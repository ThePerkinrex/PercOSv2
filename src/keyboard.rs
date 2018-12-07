use cpuio::Port;

// Index = Scancode-1
const CONFIGURED_KEYS: usize = 58;

// <ES-ES Apple Keyboard>
const KEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\'', '¡', '\0'/*BACKSPACE*/,
    '\t', 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '`', '+', '\n',
    '\0'/*CTRL*/, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ñ', '´', 'º', '\0'/*LSHIFT*/, 'ç',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const SHIFTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '!', '"', '·', '$', '%', '&', '/', '(', ')', '=', '?', '¿', '\0'/*BACKSPACE*/,
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '^', '*', '\n',
    '\0'/*CTRL*/, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ñ', '¨', 'ª', '\0'/*LSHIFT*/, 'Ç',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', ';', ':', '_', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const ALTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '|', '@', '#', '¢', '∞', '¬', '÷', '“', '”', '≠', '´', '‚', '\0'/*BACKSPACE*/,
    '\t', 'œ', 'æ', '€', '®', '†', '¥', ' ', ' ', 'ø', 'π', '[', ']', '\n',
    '\0'/*CTRL*/, 'å', '∫', '∂', 'ƒ', '', '™', '¶', '§', ' ', '~', '{', '\\', '\0'/*LSHIFT*/, '}',
    ' ', '∑', '©', '√', 'ß', ' ', 'µ', '„', '…', '–', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const ALTSHIFTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, 'ı', '˝', '•', '£', '‰', ' ', '⁄', '‘', '’', '≈', '¸', '˛', '\0'/*BACKSPACE*/,
    '\t', 'Œ', 'Æ', '€', ' ', '‡', ' ', ' ', ' ', 'Ø', '∏', 'ˆ', '±', '\n',
    '\0'/*CTRL*/, 'Å', ' ', '∆', 'ﬁ', 'ﬂ', ' ', '¯', 'ˇ', '˘', '˜', '«', '°', '\0'/*LSHIFT*/, '»',
    '‹', '›', ' ', '◊', ' ', '˙', '˚', ' ', '…', '—', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const CAPSKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\'', '¡', '\0'/*BACKSPACE*/,
    '\t', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '`', '+', '\n',
    '\0'/*CTRL*/, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ñ', '´', 'º', '\0'/*LSHIFT*/, 'Ç',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '-', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];
// </ES-ES Apple Keyboard>

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ModifierKey {
    ESC         = 01,
    LShift      = 42,
    RShift      = 54,
    Ctrl       = 29,
    Alt         = 56,
    CapsLock    = 58,
    BackSpace   = 14,
}


pub fn check_release(last_code: u8, current_code: u8) -> bool {
    if current_code > 128{
        return (last_code != current_code) && ((128 | last_code) == current_code) || (((current_code - 128)as usize) <= CONFIGURED_KEYS);
    }else{
        return (last_code != current_code) && ((128 | last_code) == current_code)
    }
}

pub fn release_press_code(release_code: u8) -> u8 {
    return release_code - 128;
}


#[derive(Debug)]
pub struct KeyHandler {
    keyboard_port: Port<u8>,
    caps: bool,
    alt: bool,
    shift: bool,
    last_keycode: u8,
}

impl KeyHandler {
    pub fn new() -> KeyHandler{
        KeyHandler {
            keyboard_port: unsafe { Port::new(0x60) },
            caps: false,
            shift: false,
            alt: false,
            last_keycode: 0,
        }
    }

    pub fn update(&mut self) -> Option<char> {
        let read = self.keyboard_port.read();
        let mut stdin: Option<char> = None;
        if read != 224 {
            
            if self.last_keycode != read {

                let released: bool = check_release(self.last_keycode, read);
                
                if released {
                    //println!("releasecode: 0x{:x?}, {:?}, {:?}", read, read, release_press_code(read));
                    let scancode = release_press_code(read);
                    if scancode <= CONFIGURED_KEYS as u8 {
                        let key: char;
                        if self.shift && self.alt{
                            key = ALTSHIFTKEYS[(scancode-1) as usize];
                        }else if self.shift{
                            key = SHIFTKEYS[(scancode-1) as usize];
                        }else if self.alt{
                            key = ALTKEYS[(scancode-1) as usize];
                        }else if self.caps{
                            key = CAPSKEYS[(scancode-1) as usize];
                        }else{
                            key = KEYS[(scancode-1) as usize];
                        }
                        if key == '\0' {
                            stdin = self.handle_mod_key_release(scancode);
                        }else{
                            stdin = Some(key.clone());
                            //print!("{}", key);
                        }
                    }
                    
                }else{
                    
                    //println!("scancode: 0x{:x?}, {:?}", read, read);

                    if read <= CONFIGURED_KEYS as u8 {
                        let key: char;
                        if self.shift && self.alt{
                            key = ALTSHIFTKEYS[(read-1) as usize];
                        }else if self.shift{
                            key = SHIFTKEYS[(read-1) as usize];
                        }else if self.alt{
                            key = ALTKEYS[(read-1) as usize];
                        }else if self.caps{
                            key = CAPSKEYS[(read-1) as usize];
                        }else{
                            key = KEYS[(read-1) as usize];
                        }
                        if key == '\0' {
                            self.handle_mod_key_press(read);
                        }
                    }
                }
            }
            self.last_keycode = read;
        }
        return stdin;
    }

    fn handle_mod_key_release(&mut self, mod_key: u8) -> Option<char>{
        //println!("Handling mod key release {}", mod_key);
        let mut stdin:Option<char> = None;
        if mod_key == ModifierKey::ESC as u8 {
            clear!();
        }else if (mod_key == ModifierKey::RShift as u8) | (mod_key == ModifierKey::LShift as u8) {
            self.shift = false;
        }else if mod_key == ModifierKey::BackSpace as u8 {
            stdin = Some('\0');
            //warnln!("No implementation for backspace");
        }else if mod_key == ModifierKey::Alt as u8 {
            self.alt = false;
        }
        return stdin;
    }

    fn handle_mod_key_press(&mut self, mod_key: u8){
        //println!("Handling mod key press {}", mod_key);
        if mod_key == ModifierKey::CapsLock as u8 {
            self.caps = !self.caps;
        }else if (mod_key == ModifierKey::RShift as u8) | (mod_key == ModifierKey::LShift as u8) {
            self.shift = true;
        }else if mod_key == ModifierKey::Alt as u8 {
            self.alt = true;
        }
    }

}
