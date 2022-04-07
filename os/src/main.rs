// os/src/main.rs

#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod safe_cell;
mod syscall;
mod trap;
mod config;
mod loader;
mod task;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("[kernel] clear_bss() Done!");
    trap::init();
    println!("[kernel] trap::init() Done!");
    task::init();
    println!("[kernel] task::init() Done!");
    task::run_app();
}

// TODO: sbss()和ebss()是什么意思？
// TODO: 了解一下rust的extern C语法
// pub fn clear_bss() {
//     extern "C" {
//         fn sbss();
//         fn ebss();
//     }
//     (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
// }

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}
