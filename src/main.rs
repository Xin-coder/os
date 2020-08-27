#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(asm)]
#![feature(global_asm)]
use core::panic::PanicInfo;
// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


global_asm!(include_str!("boot/entry64.asm"));
// 在屏幕上输出一个字符，目前我们先不用了解其实现原理
pub fn console_putchar(ch: u8) {
    let ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        asm!("ecall"
             : "={x10}" (ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}
#[no_mangle]
extern "C" fn rust_main() -> ! {
    // 在屏幕上输出 "OK\n" ，随后进入死循环
    console_putchar(b'O');
    console_putchar(b'K');
    console_putchar(b'\n');
    loop {}
}