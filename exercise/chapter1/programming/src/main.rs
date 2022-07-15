#![feature(backtrace)]

use std::fs;
use std::path::Path;
use std::backtrace::Backtrace;

fn main() {
    problem_1();

    // unsafe { problem_2(); }

    let bt = Backtrace::capture();
    println!("{:?}", bt);
}

// 1. * 实现一个linux应用程序A，显示当前目录下的文件名。（用C或Rust编程）
fn problem_1() {
    let cur_path = Path::new("./");
    print_path(cur_path, 0);
}

fn print_path(cur_path: &Path, indent: usize) {
    for entry in fs::read_dir(cur_path).expect("读取目录失败") {
        if let Ok(entry) = entry {
            let file = entry.path();
            let filename = file.to_str().unwrap();
            println!("{}{}", " ".repeat(indent), filename);

            if file.is_dir() {
                print_path(file.as_path(), indent + 4);
            }
        }
    }
}

// 2. *** 实现一个linux应用程序B，能打印出调用栈链信息。（用C或Rust编程）
// see: os-tutorial-v3/src/stack_trace.rs

// pub unsafe fn problem_2() {
//     println!("== Begin stack trace ==");
//
//     let mut base_pointer: *const usize;
//     // Get the address of pushed base pointer
//     unsafe { asm!("mov {}, rbp", out(reg) base_pointer) }
//
//     while !base_pointer.is_null() {
//         // The return address is above the pushed base pointer
//         let saved_base = *base_pointer.sub(1);
//         let saved_prev = *base_pointer.sub(2);
//
//         println!("0x{:016x}, fp = 0x{:016x}", saved_base, saved_prev);
//
//         // The pushed base pointer is the address to the previous stack frame
//         base_pointer = saved_prev as *const usize;
//     }
//
//     println!("== End stack trace ==");
// }

// 3. ** 实现一个基于rcore/ucore tutorial的应用程序C，用sleep系统调用睡眠5秒（in rcore/ucore tutorial v3: Branch ch1）
// see: os-tutorial-v3/src/
