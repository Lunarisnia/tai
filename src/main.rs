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
    writer.write_text("Hello there man\n", &ColorCode::new(Color::White, Color::Green));
    writer.write_text("Whatchu DoiSS<Z>?_@+!)_#!SS<Z>?_@+!)_#!)SS<Z>?_@+!)/__g?i    😀", &ColorCode::new(Color::White, Color::Green));
    loop {}
}