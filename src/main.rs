// Disabling standard library
#![no_std]
// Disabling default entry points
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod vga_buffer;
use core::panic::PanicInfo;
use core::fmt::Write;

static PRINT_MESSAGE: &[u8] = b"WELCOME TO OUR CUSTOM OS";

// This function invoked by the compiler when a panic occurs
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

// This attribute disables the name mangling
#[no_mangle]
// This function is the entry point, we used _start because,
// linker will look for _start function by default,
// we have used is the extern "C", this is used to tell the compiler
// to use the C calling convention for this function.
pub extern "C" fn _start() -> ! {
    //vga_buffer::print_data();
    println!("Hello test case");
    #[cfg(test)]
        test_main();
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("first test case... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

