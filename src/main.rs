// Disabling standard library
#![no_std]
// Disabling default entry points
#![no_main]

use core::panic::PanicInfo;
static PRINT_MESSAGE: &[u8] = b"WELCOME TO OUR CUSTOM OS";

// This function invoked by the compiler when a panic occurs
#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

// This attribute disables the name mangling
#[no_mangle]
// This function is the entry point, we used _start because,
// linker will look for _start function by default,
// we have used is the extern "C", this is used to tell the compiler
// to use the C calling convention for this function.
pub extern "C" fn _start() -> ! {

    let buffer = 0xb8000 as *mut u8;

    for (num, &byte) in PRINT_MESSAGE.iter().enumerate() {
        unsafe {
            *buffer.offset(num as isize * 2) = byte;
            *buffer.offset(num as isize * 2 + 1) = 0x9;
        }
    }

    // The goal is just to overwrite the entry point so we just looped indefinitely
    loop {}
}

