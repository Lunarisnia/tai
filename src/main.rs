#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::rotating_cube::RotatingCube;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_mode;
mod macros_playground;
mod rotating_cube;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("WOAH");
    print!("hey there");
    println!();
    println!("WOAH sup man");
    println!("NEW THING HERE");

    greet!("Janice", "Robbie", "James");

    RotatingCube::new().spin();
    loop {}
}