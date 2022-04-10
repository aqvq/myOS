// os/src/main.rs

#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod safe_cell;
mod sbi;
mod syscall;
mod task;
mod trap;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    println!("[kernel] Starting...");
    clear_bss();
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
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
