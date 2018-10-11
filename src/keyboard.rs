use cpuio::Port;

// Index = Scancode - 1
const keys: [Option<char>; 10] = [
    None, Some('1'), Some('2'), Some('3'), Some('4'), Some('5'), Some('6'), Some('7'), Some('8'), Some('9')
];

pub fn check_release(last_code: u8, current_code: u8) -> bool {
    return (last_code != current_code) &&(((8 << 4) | last_code) == current_code);
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ModifierKey {
    ESC         = 01,
    LShift      = 42,
    RShift      = 54,
    Ctrl        = 29,
    Alt         = 56,
    CapsLock    = 58,
}

#[derive(Debug)]
pub struct KeyHandler {
    keyboard_port: Port<u8>,
    caps: bool,
    last_keycode: u8,
}

impl KeyHandler {
    pub fn new() -> KeyHandler{
        KeyHandler {
            keyboard_port: unsafe { Port::new(0x60) },
            caps: false,
            last_keycode: 0,
        }
    }

    pub fn update(&mut self) {
        let read = self.keyboard_port.read();

        let released: bool = check_release(self.last_keycode, read);
        if self.last_keycode != read {
            if released {
                println!("releasecode: 0x{:x?}, {:?}, {:?}", read, read, ((8 << 4) | self.last_keycode));
                if read == ((8 << 4) | ModifierKey::CapsLock as u8) {
                    self.caps = !self.caps;
                    println!("Caps: {}", self.caps);
                }
            }else{
                println!("scancode: 0x{:x?}, {:?}", read, read);
            }
            
            /*if releases > 5 {
                println!("Breaking");
                break;
            }*/
        }
        self.last_keycode = read;
    }

}
