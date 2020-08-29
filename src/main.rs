#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]


mod lib;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    // extern "C" {
    //     fn _start();
    //     // fn bootstacktop();
    // }
    // println!("_start vaddr = 0x{:x}", _start as usize);
    // // println!("bootstacktop vaddr = 0x{:x}", bootstacktop as usize);
    // println!("hello world!");
    // panic!("you want to do nothing!");
    // loop {}
    println!("Hello rCore-Tutorial!");
    panic!("end of rust_main")
}