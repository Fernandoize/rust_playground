#[derive(Debug)]


enum Book {
    Papery{index: u32, name: String}, Electronic{ url: String}
}

enum Book1 {
    Papery(u32)
}

fn main() {
    let book = Book::Papery{index: 1000, name: String::from("tomorrow")};
    let ebook = Book::Electronic{url: String::from("baidu.com")};
    println!("{:?}, {:?}", book, ebook);

    // 虽然可以如此命名，但请注意，并不能像访问结构体字段一样访问枚举类绑定的属性。访问的方法在 match 语法中。
    // println!("{}", book.index);

    // 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述, rust中使用match处理，不使用switch的原因是容易忘记break;
    // match 枚举类实例 {
        // 分类1 => 返回值表达式,
        // 分类2 => 返回值表达式,
        // ...
    // }
    let result = match book {
        Book::Papery {index, name} => {
            println!("Papery book index: {}, name: {}", index, name);
            // 返回值类型必须一致
            name
        },
        Book::Electronic {url} => {
            println!("Electronic book: {}", url);
            url
        },
        _ => {
            String::from("")
        },
    };

    // 如果把枚举类附加属性定义成元组，在 match 块中需要临时指定一个名字：
    let book1 = Book1::Papery(1000);
    match book1 {
        Book1::Papery (f) => {
            println!("f: {}", f);
        }
    }

    println!("match result: {:?}", result);

    // match 除了能够对枚举类进行分支选择以外，还可以对整数、浮点数、字符和字符串切片引用（&str）类型的数据进行分支选择
    // 其中，浮点数类型被分支选择虽然合法，但不推荐这样使用，因为精度问题可能会导致分支错误。
    // 对非枚举类进行分支选择时必须注意处理例外情况，即使在例外情况下没有任何要做的事 . 例外情况用下划线 _ 表示：

    let t = "abc";
    match t {
        "abc" => {
            println!("yes");
        },
        _ => {},
    }

    // Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。
    // Rust 在语言层面彻底不允许空值 null 的存在，但无奈null 可以高效地解决少量的问题，所以 Rust 引入了 Option 枚举类：
    // enum Option<T> {
        // Some(T),
        // None,
    // }
    let opt = Option::Some("hello");
    match opt {
        Option::Some(s) => {
            println!("s is {}", s);
        },
        Option::None => {
            println!("opt is null");
        }
    }

    // &str代表字符串切片
    // str字符开始位置属性和一个字符串长度属性  
    let opt1: Option<&str> = Option::None;
    match opt1 {
        Option::Some(s) => {
            println!("s is {}", s);
        },
        Option::None => {
            println!("opt is null");
        }
    }

    // Option可以省略
    let opt2: Option<&str> = None;

    match opt2 {
        Some(s) => {},
        None => println!("opt is null")
    }

    if 1 == 2 {
        println!("true");
    }

    // if let语句
    let i: u32 = 0;
    match i {
        0 => println!(" i = 0"),
        _ => {}
    }

    // 上述可简化为
    // if let 匹配值 = 源变量 {
        // 语句块
    // }
    // if let 语法可以认为是只区分两种情况的 match 语句的"语法糖"
    let j: u32 = 0;
    if let 0 = j {
        println!("i = 0");
    }

    let book = Book::Papery{index: 1001, name: String::from("papery book1")};
    if let Book::Papery{index, name} = book {
        println!("Papery book index: {}, name: {}", index, name);
    } else {
        println!("Not papery book!")
    }

}