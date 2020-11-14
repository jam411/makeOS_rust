// main.rs

#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn painc(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}