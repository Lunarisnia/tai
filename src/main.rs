#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_mode;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("WOAH");
    print!("hey there");
    loop {}
}