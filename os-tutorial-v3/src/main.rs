#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use crate::sbi::{shutdown};

mod sbi;
mod lang_items;
mod console;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss(); // clear all bss segment to init kernel

    println!("Hello, world!");

    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" { // linker.ld provided
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
