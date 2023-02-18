
mod nation {
    pub mod goverment {
        pub fn govern() {
            println!("govern");
        }
    }
    pub mod congress {
        pub fn legistate() {
            println!("legistate");
        }   
    } 
    mod court {
        fn judical() {
            // println!("judical");
            super::congress::legistate();
        }
    }
    pub fn govern() {
        println!("outer govern");
    }

    pub use self::congress::legistate;
    // pub use self::court::judical;
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod some_module {
    pub enum Person {
        King{name: String},
        Queue
    }
}

fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("i'd like {} toast please!, seasonal_fruit: {}", meal.toast, meal.seasonal_fruit);
}

use crate::nation::goverment::govern;
use crate::nation::govern as nation_govern;

fn main() {
    // Crate 表示箱, "箱"是二进制程序文件或者库文件，存在于"包"中, "箱"是树状结构的，它的树根是编译器开始运行时编译的源文件所编译的程序。
    
    // Package 包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项
    // 一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
    // 当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，
    // Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。
    
    // 对于一个软件工程来说，我们往往按照所使用的编程语言的组织规范来进行组织，
    // 组织模块的主要结构往往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
    // 这些先进的语言的组织单位可以层层包含，就像文件系统的目录结构一样。Rust 中的组织单位是模块（Module）。




    // 二进制的文件称为Crate, cargo管理的工程称为Package, Module用来管理代码之间的组织关系

    // government是私有的，无法访问
    // crate::nation::goverment::govern();
    
    // 访问权限
    // Rust 中有两种简单的访问权：公共（public）和私有（private）。默认情况下，如果不加修饰符，模块中的成员访问权将是私有的。
    // 如果想使用公共权限，需要使用 pub 关键字。 对于私有的模块，只有在与其平级的位置或下级的位置才能访问，不能从其外部访问。

    nation::goverment::govern();

    // 如果模块中定义了结构体，结构体除了其本身是私有的以外，其字段也默认是私有的。所以如果想使用模块中的结构体以及其字段，需要 pub 声明;
    eat_at_restaurant();


    // 枚举类枚举项可以内含字段，但不具备类似的性质:
    let person = some_module::Person::King{
        name: String::from("Blue")
    };
    match person {
        some_module::Person::King{name} => {
            println!("{}", name);
        },
        _ => {}
    }

    govern();
    nation_govern();
    nation::legistate();
    // judical是私有的，无法被调用
    // nation::judical();

    use std::f64::consts::PI;
    println!("{}, {}", PI, (PI / 2.0).sin());
}