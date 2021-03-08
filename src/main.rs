// Disabling standard library
#![no_std]
// Disabling default entry points
#![no_main]
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





/*let buffer = 0xb8000 as *mut u8;

    for (num, &byte) in PRINT_MESSAGE.iter().enumerate() {
        unsafe {
            *buffer.offset(num as isize * 2) = byte;
            *buffer.offset(num as isize * 2 + 1) = 0x9;
        }
    }*/

//vga_buffer::print_something();
//use core::fmt::Write;
//vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
//write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
/*println!("Hello World{}", "!");
panic!("Some panic message");
*/
