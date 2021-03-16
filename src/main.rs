// Disabling standard library
#![no_std]
// Disabling default entry points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

// This function invoked by the compiler when a panic occurs
#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    //println!(panic_info);
    loop {}
}

// This attribute disables the name mangling
#[no_mangle]
// This function is the entry point, we used _start because,
// linker will look for _start function by default,
// we have used is the extern "C", this is used to tell the compiler
// to use the C calling convention for this function.
pub extern "C" fn _start() -> ! {
    vga_buffer::print_data();
    loop {}
}
