    .section .text.entry
    .globl _start
_start:
    la sp, bootstacktop #赋值sp栈顶指针
    call rust_main      #跳转至rust_main函数

    .section .bss.stack
    .align 12
    .global bootstack
bootstack:
    .space 4096 * 4
    .global bootstacktop
bootstacktop: