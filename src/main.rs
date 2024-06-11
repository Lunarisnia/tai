#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut CURRENT_OFFSET: isize = 0;

fn safe_print(color: ColorCode, str: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    let mut last_index: isize = 0;
    for (i, &byte) in str.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.offset(CURRENT_OFFSET + (i as isize * 2)) = byte;
            *vga_buffer.offset(CURRENT_OFFSET + (i as isize * 2 + 1)) = color.0;
        }
        last_index = i as isize + 1;
    }
    unsafe {
        CURRENT_OFFSET = CURRENT_OFFSET + (last_index * 2);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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
struct ColorCode(u8);

impl ColorCode {
    pub fn new(background: Color, foreground: Color) -> ColorCode {
        return ColorCode((background as u8) << 4 | foreground as u8);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    safe_print(ColorCode::new(Color::Red, Color::Blue), "Hey");
    loop {}
}