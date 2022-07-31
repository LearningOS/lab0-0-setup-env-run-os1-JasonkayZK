#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use log::{debug, error, info, trace, warn};

#[no_mangle]
fn main() -> i32 {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn start_bss();
        fn end_bss();
    }

    user_lib::logger::init();
    println!("Hello, world!");

    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(".bss [{:#x}, {:#x})", start_bss as usize, end_bss as usize);
    error!("no error");

    0
}
