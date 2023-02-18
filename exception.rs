
// 程序中一般会出现两种错误：可恢复错误和不可恢复错误。
// 可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。
// 但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。
// Rust中没有Exception 对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

use std::io;
use std::io::Read;
use std::fs::File;


fn main() {
    // panic!("error occured!");
    // println!("Hello Rust");

    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("file open successful, file: {:?}", file);
        },
        Err(err) => {
            println!("Failed to open file, err: {}", err);
        }
    }

    let f = File::open("hello.txt");
    // if let简化版本
    if let Ok(file) = f {
        println!("file open successful, file: {:?}", file);
    } else {
        println!("Failed to open file");
    }

    // 将可恢复错误按照不可回复错误处理, 也就是类似于java中捕捉到了已知错误，但是此时想要按照不可回复错误panic!抛出，便于让程序终止
    // 如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：

    // let f1 = File::open("hello.txt").unwrap();
    // let f2 = File::open("hello.txt").expect("Failed to open file");

    let r = fu(100);
    if let Ok(i) = r {
        println!("Ok: f(10000) = {}", i);
    } else {
        println!("Err");
    }

    // Kind方法： Rust 似乎没有像 try 块一样可以令任何位置发生的同类异常都直接得到相同的解决的语法
    // ，但这样并不意味着 Rust 实现不了：我们完全可以把 try 块在独立的函数中实现，将所有的异常都传递出去解决。
    // 实际上这才是一个分化良好的程序应当遵循的编程方法：应该注重独立功能的完整性。
    // 但是这样需要判断 Result 的 Err 类型，获取 Err 类型的函数是 kind()。
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Can not read file");
                }
            }
        }
    }

}

// 自己编写一个处理错误的函数
fn fu(i: i32) -> Result<i32, bool> {
    if i > 0 { Ok(i) } 
    else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool> {
    // let t = fu(i);
    // return match t {
    //     Ok(i) => Ok(i),
    //     Err(i) => Err(i)
    // };

    // 简化版
    // Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去
    // ? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
    let t = fu(i)?;
    Ok(t)
}

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let f = File::open(path);
    let mut s = String::new();
    f?.read_to_string(&mut s);
    Ok(s)
}


