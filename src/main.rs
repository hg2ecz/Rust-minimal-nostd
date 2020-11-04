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
    fn exit(status: i32) -> !;
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let s = b"Hello, World!\n";
    unsafe {
        write(1, s as *const u8 as *const i8, s.len());
        exit(0);
    }
    // or loop()
}
