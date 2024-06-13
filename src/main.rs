#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga_mode::{Color, ColorCode, Writer};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_mode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // safe_print(ColorCode::new(Color::Red, Color::Blue), "Hello, Wolrd");
    let mut writer: Writer = Writer::new();
    writer.write_byte(&b'x', &ColorCode::new(Color::White, Color::Red));
    loop {}
}