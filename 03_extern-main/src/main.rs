#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link(name = "c")]
extern "C" {
    fn write(fd: i32, buf: *const i8, count: usize) -> isize;
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    let s = "Hello, World!\n";
    unsafe {
        write(1, s.as_ptr() as *const i8, s.len());
    }
    42
}
