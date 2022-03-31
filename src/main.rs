// os/src/main.rs

#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
use core::{arch::global_asm, panic};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();

    println!("Hello World!");
    panic!("Shutdown machine!");
}

#[macro_use]
mod console;

// TODO: sbss()和ebss()是什么意思？
// TODO: 了解一下rust的extern C语法
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
