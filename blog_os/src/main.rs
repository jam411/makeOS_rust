// main.rs

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn painc(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {

}
