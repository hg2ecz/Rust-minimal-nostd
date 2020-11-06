#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    // Since we are passing a C string the final null character is mandatory
    let hello = "Hello, World!\n\0";

    unsafe {
        libc::printf(hello.as_ptr() as *const _);
    }

    let fname = "helloworld.txt\0".as_ptr() as *const _;
    let mode = "w\0".as_ptr() as *const _;

    unsafe {
        let file = libc::fopen(fname, mode);
        libc::fprintf(file, hello.as_ptr() as *const _);
        libc::fclose(file);
    }
    42
}
