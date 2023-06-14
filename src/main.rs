#![no_main]
#![no_std]

use core::panic::PanicInfo;
mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World, {}!", "Daniel");
    loop {}
}
