#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

const BASE_COLOR_CODE: ColorCode = ColorCode::new(Color::White, Color::Black);
const WARN_COLOR_CODE: ColorCode = ColorCode::new(Color::Yellow, Color::Black);
const PANIC_COLOR_CODE: ColorCode = ColorCode::new(Color::LightRed, Color::Black);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

use volatile::Volatile;

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

use core::ptr::Unique;

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' => self.backspace(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn backspace(&mut self) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let col = self.column_position;
        if col != 0 {
            self.buffer().chars[BUFFER_HEIGHT-1][col-1].write(blank);
            self.column_position -= 1;
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
    }
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

use spin::Mutex;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: BASE_COLOR_CODE,
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});

#[allow(unused_macros)]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[allow(unused_macros)]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[allow(unused_macros)]
macro_rules! panic_print {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::panic_print(format_args!($($arg)*));
    });
}

pub fn panic_print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().color_code = PANIC_COLOR_CODE;
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().color_code = BASE_COLOR_CODE;
}

#[allow(unused_macros)]
macro_rules! warnln {
    ($fmt:expr) => (warn!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (warn!(concat!($fmt, "\n"), $($arg)*));
}

#[allow(unused_macros)]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::vga_buffer::warn(format_args!($($arg)*));
    });
}

pub fn warn(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().color_code = WARN_COLOR_CODE;
    WRITER.lock().write_fmt(args).unwrap();
    WRITER.lock().color_code = BASE_COLOR_CODE;
}

#[allow(unused_macros)]
macro_rules! clear {
    () => ({
        $crate::vga_buffer::clear_screen();
    });
}

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}
