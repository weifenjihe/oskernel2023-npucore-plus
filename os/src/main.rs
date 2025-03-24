#![no_std]
#![no_main]
#![feature(asm)]
//#![feature(asm_const)]
#![feature(core_intrinsics)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(int_roundings)]
#![feature(string_remove_matches)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(option_result_unwrap_unchecked)]
#![feature(const_maybe_uninit_assume_init)]
pub use arch::config;
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
mod console;
mod arch;
mod drivers;
mod fs;
mod lang_items;
mod mm;
mod syscall;
mod task;
mod timer;

pub use arch::trap;

use crate::arch::{bootstrap_init, machine_init};

#[cfg(feature = "rv64")]
core::arch::global_asm!(include_str!("arch/rv64/entry.asm"));
#[cfg(feature = "comp")]
core::arch::global_asm!(include_str!("preload_app.S"));


fn mem_clear() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    #[cfg(feature = "zero_init")]
    unsafe {
        core::slice::from_raw_parts_mut(
            sbss as usize as *mut u8,
            crate::config::MEMORY_END - sbss as usize,
        )
        .fill(0);
    }
    #[cfg(not(feature = "zero_init"))]
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> isize {
    bootstrap_init();

    mem_clear();
    console::log_init();
    println!("[kernel] Console initialized.");
    mm::init();
    mm::remap_test();

    machine_init();
    println!("[kernel] Hello, world!");

    //machine independent initialization
    fs::directory_tree::init_fs();
    #[cfg(feature = "comp")]
    fs::flush_preload();
    task::add_initproc();
    task::run_tasks();
    // panic!("Unreachable in rust_main!");
    0
}

#[cfg(test)]
fn test_runner(_tests: &[&dyn Fn()]) {}
