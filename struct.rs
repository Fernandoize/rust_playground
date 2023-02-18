// 导入debug库，便于调试
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // Rust 语言不是面向对象的，从它所有权机制的创新可以看出这一点。但是面向对象的珍贵思想可以在 Rust 实现。
    // 结构体方法的第一个参数必须是 &self，不需声明类型，因为 self 不是一种风格而是关键字。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: Rectangle) -> bool {
        self.width > rect.width
    }

    // 关联函数，这种函数不依赖实例，但是使用它需要声明是在哪个 impl 块中的。
    fn create_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {

    //  Rust 中的结构体（Struct）与元组（Tuple）都可以将若干个类型不一定相同的数据捆绑在一起形成整体
    //，但结构体的每个成员和其本身都有一个名字，这样访问它成员的时候就不用记住下标了。
    // 元组常用于非定义的多值传递，而结构体用于规范常用的数据结构。结构体的每个成员叫做"字段"。

    //  Rust 里 struct 语句仅用来定义，不能声明实例
    struct Site {
        domain: String,
        name: String,
        nation: String,
        found: i32
    }

    let domain = String::from("www.baidu.com");
    let name = String::from("name");
    let runoob = Site {
        domain,
        name,
        nation: String::from("China"),
        found: 2013
    };

    // ..runoob 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，
    // 意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    // let _site = Site {
    //     domain: String::from("www.runoob.com"),
    //     name: String::from("RUNOOB"),
    //     ..runoob
    // };
    println!("{}, {}, {}, {}", runoob.domain, runoob.name, runoob.nation, runoob.found);


    // 元组结构体是一种形式是元组的结构体。
    // 与元组的区别是它有名字和固定的类型格式。
    // 它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0,0,0);
    let point = Point(1.0, 2.0);
    println!("color: ({}, {}, {})", black.0, black.1, black.2);
    println!("point: ({}, {})", point.0, point.1);

    // 结构体必须掌握字段值所有权，因为结构体失效的时候会释放所有字段。
    // 在结构体中使用str，当结构体失效时可能会出现基本类型还未释放


    let rect1 = Rectangle { width: 30, height: 50};
    let rect2 = Rectangle { width: 40, height: 50};
    println!("rect1 is {:?}", rect1);
    println!("rect1 area is {}", rect1.area());
    println!("rect1 is wider, {}", rect1.wider(rect2));

    println!("{:?}", Rectangle::create_rectangle(10, 20));
}