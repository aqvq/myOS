// os/src/main.rs

//! myOS

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![deny(missing_docs)]
// #![deny(warnings)]

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
mod timer;
#[cfg(not(any(feature = "board_k210")))]
#[path = "boards/qemu.rs"]
mod board;
#[cfg(feature = "board_k210")]
#[path = "boards/k210.rs"]
mod board;

use core::include_str;
use core::arch::global_asm;
global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
/// main function entry
pub fn rust_main() -> ! {
    println!("[kernel] Starting...");
    clear_bss();
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_triger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}

/// clear bss segment
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
