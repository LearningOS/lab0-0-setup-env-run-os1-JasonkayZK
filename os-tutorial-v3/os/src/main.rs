//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`batch::run_next_app()`] and for the first time go to
//! userspace.

#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};
use crate::sbi::{shutdown};

// #[cfg(feature = "board_qemu")]
// #[path = "boards/qemu.rs"]
// mod board;

#[macro_use]
// pub mod batch;
// pub mod syscall;
// pub mod trap;
// mod sync;
mod console;
mod lang_items;
mod sbi;
mod stack_trace;
mod logger;

// boot & load os for qemu
global_asm!(include_str!("entry.asm"));
// load user app for os
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    extern "C" {
        // linker.ld provided
        fn sbss(); // start of bss address
    fn ebss(); // end of bss address
    }

    // clear all bss segment
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {
            (a as *mut u8).write_volatile(0)
        }
    });
}

#[no_mangle]
pub fn main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss(); // clear all bss segment to init kernel

    logger::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}
