// main.rs

#![no_std] // Disable the Rust standard library
#![no_main] // Overwrite the Entry point

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Entrypoint to be called using C calling convention.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
