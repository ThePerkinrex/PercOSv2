use cpuio::Port;

// Index = Scancode-1
const CONFIGURED_KEYS: usize = 58;

// <ES-ES Apple Keyboard>
const KEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\'', '¡', '\0'/*BACKSPACE*/,
    '\0'/*TAB*/, 'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', '`', '+', '\0'/*ENTER*/,
    '\0'/*CTRL*/, 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'ñ', '´', 'º', '\0'/*LSHIFT*/, 'ç',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '-', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const SHIFTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '!', '"', '·', '$', '%', '&', '/', '(', ')', '=', '?', '¿', '\0'/*BACKSPACE*/,
    '\0'/*TAB*/, 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '^', '*', '\0'/*ENTER*/,
    '\0'/*CTRL*/, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ñ', '¨', 'ª', '\0'/*LSHIFT*/, 'Ç',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', ';', ':', '_', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const ALTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '|', '@', '#', '¢', '∞', '¬', '÷', '“', '”', '≠', '´', '‚', '\0'/*BACKSPACE*/,
    '\0'/*TAB*/, 'œ', 'æ', '€', '®', '†', '¥', ' ', ' ', 'ø', 'π', '[', ']', '\0'/*ENTER*/,
    '\0'/*CTRL*/, 'å', '∫', '∂', 'ƒ', '', '™', '¶', '§', ' ', '~', '{', '\\', '\0'/*LSHIFT*/, '}',
    ' ', '∑', '©', '√', 'ß', ' ', 'µ', '„', '…', '–', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const ALTSHIFTKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, 'ı', '˝', '•', '£', '‰', ' ', '⁄', '‘', '’', '≈', '¸', '˛', '\0'/*BACKSPACE*/,
    '\0'/*TAB*/, 'Œ', 'Æ', '€', ' ', '‡', ' ', ' ', ' ', 'Ø', '∏', 'ˆ', '±', '\0'/*ENTER*/,
    '\0'/*CTRL*/, 'Å', ' ', '∆', 'ﬁ', 'ﬂ', ' ', '¯', 'ˇ', '˘', '˜', '«', '°', '\0'/*LSHIFT*/, '»',
    '‹', '›', ' ', '◊', ' ', '˙', '˚', ' ', '…', '—', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];

const CAPSKEYS: [char; CONFIGURED_KEYS] = [
    '\0'/*ESC*/, '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\'', '¡', '\0'/*BACKSPACE*/,
    '\0'/*TAB*/, 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', '`', '+', '\0'/*ENTER*/,
    '\0'/*CTRL*/, 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ñ', '´', 'º', '\0'/*LSHIFT*/, 'Ç',
    'Z', 'X', 'C', 'V', 'B', 'N', 'M', ',', '.', '-', '\0'/*RSHIFT*/,
    '\0'/* LCMD? */,'\0'/* ALT */,' ', '\0' /* RCMD? */
];
// </ES-ES Apple Keyboard>

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ModifierKey {
    ESC         = 01,
    BackSpace   = 14,
    Tab         = 15,
    Enter       = 28,
    Ctrl        = 29,
    LShift      = 42,
    RShift      = 54,
    Alt         = 56,
    CapsLock    = 58,
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

type ModFlagsType = u8;

pub const KEY_ENTER     :ModFlagsType = 1<<0;
pub const KEY_TAB       :ModFlagsType = 1<<1;
pub const KEY_BACKSPACE :ModFlagsType = 1<<2;
pub const KEY_ESC       :ModFlagsType = 1<<3;

#[derive(Debug, Clone)]
pub struct KeyHandlerOut {
    stdin: char,
    mod_flags: ModFlagsType,
}

impl KeyHandlerOut {
    pub fn new(stdin: char) -> KeyHandlerOut {
        KeyHandlerOut {
            stdin: stdin,
            mod_flags: 0 as ModFlagsType
    ,
        }
    }

    pub fn get_flag(self, flag: ModFlagsType) -> bool {
        ((self.mod_flags & flag) != 0)
    }

    pub fn set_flag(&mut self, flag: ModFlagsType) {
        self.mod_flags |= flag;
    }

    pub fn unset_flag(&mut self, flag: ModFlagsType) {
        self.mod_flags -= flag;
    }

    pub fn get_stdin(self) -> char {
        self.stdin
    }

    pub fn set_stdin(&mut self, new_stdin: char) {
        self.stdin = new_stdin;
    }
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

    pub fn update(&mut self) -> Option<KeyHandlerOut> {
        let read = self.keyboard_port.read();
        let mut out: Option<KeyHandlerOut> = None;
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
                            out = self.handle_mod_key_release(scancode);
                        }else{
                            out = Some(KeyHandlerOut::new(key.clone()));
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
        return out;
    }

    fn handle_mod_key_release(&mut self, mod_key: u8) -> Option<KeyHandlerOut>{
        //println!("Handling mod key release {}", mod_key);
        let mut out: Option<KeyHandlerOut> = None;
        if mod_key == ModifierKey::ESC as u8 {
            let mut key_out = KeyHandlerOut::new('\0');
            key_out.set_flag(KEY_ESC);
            out = Some(key_out);
        }else if (mod_key == ModifierKey::RShift as u8) | (mod_key == ModifierKey::LShift as u8) {
            self.shift = false;
        }else if mod_key == ModifierKey::BackSpace as u8 {
            let mut key_out = KeyHandlerOut::new('\x08');
            key_out.set_flag(KEY_BACKSPACE);
            out = Some(key_out);
        }else if mod_key == ModifierKey::Alt as u8 {
            self.alt = false;
        }else if mod_key == ModifierKey::Enter as u8 {
            let mut key_out = KeyHandlerOut::new('\n');
            key_out.set_flag(KEY_ENTER);
            out = Some(key_out);
        }else if mod_key == ModifierKey::Tab as u8 {
            let mut key_out = KeyHandlerOut::new('\t');
            key_out.set_flag(KEY_TAB);
            out = Some(key_out);
        }
        return out;
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
