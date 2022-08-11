#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

use crate::syscall::sys_get_time;

#[macro_use]
pub mod console;
pub mod logger;
mod lang_items;
mod syscall;
mod stack_trace;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    syscall::sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    syscall::sys_exit(exit_code)
}

pub fn yield_() -> isize { syscall::sys_yield() }

pub fn get_time() -> isize {
    sys_get_time()
}

// The real entry for main
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("can not find main!");
}

fn clear_bss() {
    extern "C" {
        // linker.ld provided
        fn start_bss(); // start of bss address
    fn end_bss(); // end of bss address
    }

    // clear all bss segment
    (start_bss as usize..end_bss as usize).for_each(|a| unsafe {
        (a as *mut u8).write_volatile(0)
    });
}
