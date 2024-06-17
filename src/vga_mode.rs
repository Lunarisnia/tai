use core::fmt;
use core::fmt::{Arguments, Write};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!(
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(ColorCode::new(Color::Black, Color::White)));
);

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(background: Color, foreground: Color) -> ColorCode {
        return ColorCode((background as u8) << 4 | foreground as u8);
    }
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub struct Writer {
    column_position: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(color: ColorCode) -> Writer {
        Writer {
            column_position: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            color,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_text(&mut self, text: &str) {
        for (_, &b) in text.as_bytes().iter().enumerate() {
            match &b {
                0x20..=0x7e | b'\n' => self.write_byte(b),
                &_ => self.write_text("*"),
            }
        }
    }

    fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                self.buffer.chars[r - 1][c] = self.buffer.chars[r][c];
            }
        }
        self.clear_line(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_line(&mut self, row: usize) {
        self.buffer.chars[row] = [ScreenChar {
            ascii_character: b' ',
            color_code: self.color,
        }; BUFFER_WIDTH];
    }

    fn clear(&mut self) {
        self.buffer.chars = [[ScreenChar {
            ascii_character: b' ',
            color_code: self.color,
        }; BUFFER_WIDTH]; BUFFER_HEIGHT];
    }
}


impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_text(&s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::vga_mode::_print(format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n",format_args!($($arg)*))
    }
}

#[macro_export]
macro_rules! clear {
    () => {
        $crate::vga_mode::_clear();
    };
}

pub fn _clear() {
    WRITER.lock().clear();
}

pub fn _print(args: Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap()
}

// how to do it raw
// fn safe_print(color: ColorCode, str: &str) {
//     // let vga_buffer = 0xb8000 as *mut u8;
//     let vga_buffer = unsafe { &mut *(0xb8000 as *mut Buffer) };
//
//     // let mut last_index: isize = 0;
//     for (i, &byte) in str.as_bytes().iter().enumerate() {
//         vga_buffer.chars[i] = ScreenChar{
//             ascii_character: byte,
//             color_code: color,
//         };
//     }
//     // unsafe {
//     //     CURRENT_OFFSET = CURRENT_OFFSET + (last_index * 2);
//     // }
// }