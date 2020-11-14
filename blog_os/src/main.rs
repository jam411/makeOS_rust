// main.rs

#![no_std]

fn main() {}

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn painc(_info: &PanicInfo) -> ! {
    loop {}
}
