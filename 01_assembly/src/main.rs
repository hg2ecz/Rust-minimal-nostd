#![feature(asm)] // asm: nightly build
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn asm_write(s: &str) {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "syscall",
            in("rax") 1, // syscall number
            in("rdi") 1, // fd (stdout)
            in("rsi") s.as_ptr(),
            in("rdx") s.len(),
            //lateout("rax") ret,
        );
        #[cfg(target_arch = "aarch64")]
        asm!(
            "svc 0",
            in("w8") 0x40, // syscall number
            in("x0") 1,    // fd (stdout)
            in("x1") s.as_ptr(),
            in("x2") s.len(),
            //lateout("w8") ret,
        );
    }
}

fn asm_exit(retval: i64) -> ! {
    unsafe {
        #[cfg(target_arch = "x86_64")]
        asm!(
            "syscall",
            in("rax") 0x3c,   // system call number (sys_exit)
            in("rdi") retval, // exit value
        );
        #[cfg(target_arch = "aarch64")]
        asm!(
            "svc 0",
            in("w8") 0x5d,   // system call number (sys_exit)
            in("x0") retval, // exit value
        );
    }
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let s = "Hello, World!\n";
    asm_write(&s);
    asm_exit(42);
}
