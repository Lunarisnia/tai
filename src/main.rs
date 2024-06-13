#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga_mode::{Buffer, Color, ColorCode, ScreenChar};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut CURRENT_OFFSET: isize = 0;

fn safe_print(color: ColorCode, str: &str) {
    // let vga_buffer = 0xb8000 as *mut u8;
    let vga_buffer = unsafe { &mut *(0xb8000 as *mut Buffer) };

    // let mut last_index: isize = 0;
    for (i, &byte) in str.as_bytes().iter().enumerate() {
        vga_buffer.chars[i] = ScreenChar{
            ascii_character: byte,
            color_code: color,
        };
    }
    // unsafe {
    //     CURRENT_OFFSET = CURRENT_OFFSET + (last_index * 2);
    // }
}
mod vga_mode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    safe_print(ColorCode::new(Color::Red, Color::Blue), "Hello, Wolrd");
    loop {}
}