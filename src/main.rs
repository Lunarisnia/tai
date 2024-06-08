#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut CURRENT_OFFSET: isize = 0;

fn safe_print(str: &str) {
    let vga_buffer = 0xb8000 as *mut u8;

    let mut last_index: isize = 0;
    for (i, &byte) in str.as_bytes().iter().enumerate() {
        unsafe {
            *vga_buffer.offset(CURRENT_OFFSET + (i as isize * 2)) = byte;
            *vga_buffer.offset(CURRENT_OFFSET + (i as isize * 2 + 1)) = 0xb;
        }
        last_index = i as isize + 1;
    }
    unsafe {
        CURRENT_OFFSET = CURRENT_OFFSET + (last_index * 2);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    safe_print("Fuck Me Man Woah this somehow worked ");
    safe_print("well enough that I can do this");
    loop {}
}
