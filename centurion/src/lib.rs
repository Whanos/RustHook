#![no_std]

use core::panic::PanicInfo;

fn main() {
    
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Driver entry point
#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0 /* STATUS_SUCCESS */
}