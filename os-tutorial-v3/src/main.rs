#![no_std]
#![no_main]

use core::arch::global_asm;
use crate::sbi::{console_putchar, shutdown};

mod sbi;
mod lang_items;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss(); // clear all bss segment to init kernel

    for c in "OK\n".bytes() {
        console_putchar(c as usize);
    }

    shutdown()
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
