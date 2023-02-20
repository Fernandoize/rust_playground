use std::fmt::Display;

fn main() {


    // Rust 生命周期机制是与所有权机制同等重要的资源管理机制。
    // 之所以引入这个概念主要是应对复杂类型系统中资源管理的问题。
    // 引用是对待复杂类型时必不可少的机制，毕竟复杂类型的数据不能被处理器轻易地复制和计算。
    // 但引用往往导致极其复杂的资源管理问题，首先认识一下垂悬引用：
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // // 'x 比 'r的声明周期小，引用必须在值的生命周期以内才有效。
    // println!("r = {}", r);

    let r; 
    {
        let s1 = "rust";
        let s2 = "python";

        // 但对于函数来说，它并不能知道自己以外的地方是什么情况，
        // 它为了保障自己传递出去的值是正常的，必选所有权原则消除一切危险，所以 longer 函数并不能通过编译。
        r = longer(&s1, &s2);

        // s1, s2位基本类型，在栈内copy,因此此处可用
        // println!("{}, {}", s1, s2);
    }

    println!("{} is longer", r);


    // 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段，但str是存储在栈中，无法随着结构体立即释放
    // &str代表切片
    // 表示content的生命周期和struct的生命周期一致
    struct Str<'a> {
        content: &'a str
    }

    impl<'a> Str<'a> {
        fn get_content(&self) -> &str {
            self.content
        }
    }

    let s = Str {
        content: "test"
    };
    println!("{}", s.content);

    longest_with_an_anouncement("longer", "short", "test");


    // 静态声明周期
    // 生命周期注释有一个特别的：'static 。
    // 所有用双引号包括的字符串常量所代表的精确数据类型都是 &'static str ，
    // 'static 所表示的生命周期从程序运行开始到程序运行结束

}

// 泛型、特性与生命周期协同作战
fn longest_with_an_anouncement<'a, T>(x: &'a str, y: &'a str, ann:T) -> &'a str 
    where T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 生命周期注释是描述引用生命周期的办法。
// 虽然这样并不能够改变引用的生命周期，但可以在合适的地方声明两个引用的生命周期一致。
// 生命周期注释用单引号开头，跟着一个小写字母单词：

// &i32        // 常规引用
// &'a i32     // 含有生命周期注释的引用
// &'a mut i32 // 可变型含有生命周期注释的引用
// 我们需要用泛型声明来规范生命周期的名称，
// 随后函数返回值的生命周期将与两个参数的生命周期一致
// 所以在调用时可以这样写：
fn longer<'b>(s1: &'b str, s2: &'b str) -> &'b str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}