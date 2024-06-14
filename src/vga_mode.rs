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
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            column_position: 0,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: &u8, color: &ColorCode) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: *byte,
                    color_code: *color,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_text(&mut self, text: &str, color: &ColorCode) {
        for (_, &b) in text.as_bytes().iter().enumerate() {
            match &b {
                0x20..=0x7e | b'\n' => self.write_byte(&b, color),
                &b => self.write_text("*", color),
            }
        }
    }

    fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                self.buffer.chars[r - 1][c] = self.buffer.chars[r][c];
            }
        }
        self.buffer.chars[BUFFER_HEIGHT - 1] = [ScreenChar {
            ascii_character: 0,
            color_code: ColorCode(0),
        }; BUFFER_WIDTH];
        self.column_position = 0;
    }
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
