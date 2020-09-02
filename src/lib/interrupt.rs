use riscv::register::{
    scause::{
        self,
        Trap,
        Exception,
        Interrupt
    },
    sepc,
    stvec,
    sscratch,
    sstatus,
};
use super::timer::{
    TICKS,
    clock_set_next_event
};
use super::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            // 中断处理总入口
            fn __alltraps();
        }
        // 由于现在是在内核态
        // 我们要把 sscratch 初始化为 0
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        // 设置 sstatus 的 SIE 位
        sstatus::set_sie();
    }
    println!("++++ setup interrupt! ++++");
}

// fn trap_handler() -> ! {
//     let cause = scause::read().cause();
//     let epc = sepc::read();
//     println!("trap: cause: {:?}, epc: 0x{:#x}", cause, epc);
//     panic!("trap handled!");
// }


// 以 &mut TrapFrame 作为参数，因此可以知道中断相关信息
// 在这里进行中断分发及处理
#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    //println!("rust_trap!");
    match tf.scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => undefined_trap(tf)
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer()->(){
    // 设置下一次时钟中断触发时间
    clock_set_next_event();
    unsafe {
        // 更新时钟中断触发计数
        // 注意由于 TICKS 是 static mut 的
        // 后面会提到，多个线程都能访问这个变量
        // 如果同时进行 +1 操作，会造成计数错误或更多严重bug
        // 因此这是 unsafe 的，不过目前先不用管这个
        TICKS += 1;
        // 每触发 100 次时钟中断将计数清零并输出
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *");
        }
    }
    // 发生外界中断时，epc 指向的指令没有完成执行，因此这里不需要修改 epc
}

fn undefined_trap(tf: &mut TrapFrame)-> (){
    println!("{:?}",tf.scause.cause());
    panic!("Undefined trap!");
}