use std::io::stdin;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;


fn main() {
    // 接收命令行参数
    // 在很多语言中（如 Java 和 C/C++）环境参数是以主函数的参数（常常是一个字符串数组）传递给程序的，
    // 但在 Rust 中主函数是个无参函数，环境参数需要开发者通过 std::env 模块取出，过程

    let args = std::env::args();
    // println!("{:?}", args);
    for arg in args {
        println!("{}", arg);
    }

    // 命令行输入
    let mut str_buf = String::new();
    stdin().read_line(&mut str_buf)
    .expect("Failed to read line");
    println!("inuput line is {}", str_buf);

    // 文件读取
    // let text = fs::read("./base.rs").unwrap();
    // println!("{:?}", text);

    // 按流读取
    let mut buffer = [0u8, 5];
    // std::fs::File 的 open 方法是"只读"打开文件，并且没有配套的 close 方法，因为 Rust 编译器可以在文件不再被使用时自动关闭文件。
    let mut file = fs::File::open("./base.rs").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    // 文件写入
    fs::write("./write.txt", "im learning rust!!!").unwrap();

    // 流方式写入文件
    // 注意：打开的文件一定存放在可变的变量中才能使用 File 的方法！
    let mut file = File::create("./write.txt").unwrap();
    file.write(b"i like rust").unwrap();

    //以特定方式打开文件
    let mut file = OpenOptions::new().append(true).open("./write.txt").unwrap();
    file.write(b"today is good day").unwrap();
}