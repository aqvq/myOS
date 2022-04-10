// os/src/main.rs

//! myOS

#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![deny(missing_docs)]
#![deny(warnings)]

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

use core::include_str;
use core::arch::global_asm;
global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
/// main function entry
pub fn rust_main() -> ! {
    clear_bss();
    trap::init();
    task::init();
    loader::load_apps();
    task::run_next_app();
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
