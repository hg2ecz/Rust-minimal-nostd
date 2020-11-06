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
    fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    fn exit(status: i32) -> !;
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let s = "Hello, World!\n";
    unsafe {
        write(1, s.as_ptr(), s.len());
        exit(42);
    }
    // or loop()
}
