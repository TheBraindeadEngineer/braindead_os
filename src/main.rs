#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> !{
    braindead_os::init();

    println!("Welcome to Braindead OS!");
    braindead_os::hlt_loop();
}
