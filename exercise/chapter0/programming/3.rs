// ** 在Linux环境下编写一个可以睡眠5秒后打印出一个字符串，并把字符串内容存入一个文件中的应用程序A。(基于C或Rust语言)

use std::{thread, time};
use std::fs;

fn main() {
    thread::sleep(time::Duration::from_secs(5));

    let hello = "Hello Linux!\n";
    println!("{}", hello);

    fs::write("3.txt", hello).unwrap();
}
