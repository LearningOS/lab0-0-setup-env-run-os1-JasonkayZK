//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`task`]: Task management
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`task::run_first_task()`] and for the first time go to
//! userspace.

#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::{debug};
use crate::sbi::{shutdown};

#[cfg(feature = "board_qemu")]
#[path = "boards/qemu.rs"]
mod board;

#[macro_use]
pub mod config;
pub mod loader;
pub mod syscall;
pub mod trap;
pub mod task;
mod sync;
mod console;
mod lang_items;
mod sbi;
mod stack_trace;
mod logger;
mod timer;

// boot & load os for qemu
global_asm!(include_str!("entry.asm"));
// load user app for os
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    // linker.ld provided
    extern "C" {
        // start of bss address
        fn sbss();
        // end of bss address
        fn ebss();
    }

    // clear all bss segment
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    });
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss(); // clear all bss segment to init kernel
    logger::init();

    debug!("[kernel] Hello, world!");
    trap::init();
    debug!("[kernel] Init trap success!");
    loader::load_apps();
    debug!("[kernel] Load apps success!");
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
}
