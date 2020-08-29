use core::panic::PanicInfo;
use super::sbi::shutdown;

/// 打印 panic 的信息并 [`shutdown`]

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {

    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown()
}

/// 终止程序
/// 调用 [`panic_handler`]
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()")
}