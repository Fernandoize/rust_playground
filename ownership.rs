/*
    所有权对大多数开发者而言是一个新颖的概念，它是 Rust 语言为高效使用内存而设计的语法机制。
    所有权概念是为了让 Rust 在编译阶段更有效地分析内存资源的有用性以实现内存管理而诞生的概念。
    Rust使用所有权的概念替换java中的垃圾回收和c++中的手动回收内存

    规则如下
    1. Rust 中的每个值都有一个变量，称为其所有者。
    2. 一次只能有一个所有者。
    3. 当所有者不在程序运行范围时，该值将被删除


    如果我们定义了一个变量并给它赋予一个值，这个变量的值存在于内存中。
    这种情况很普遍。但如果我们需要储存的数据长度不确定（比如用户输入的一串字符串），
    我们就无法在定义时明确数据长度，也就无法在编译阶段令程序分配固定长度的内存空间供数据储存使用。
    （有人说分配尽可能大的空间可以解决问题，但这个方法很不文明）。这就需要提供一种在程序运行时程序自己申请使用内存的机制——堆。
    本章所讲的所有"内存资源"都指的是堆所占用的内存空间。
    有分配就有释放，程序不能一直占用某个内存资源。因此决定资源是否浪费的关键因素就是资源有没有及时的释放。
    
    Rust 之所以没有明示释放的步骤是因为在变量范围结束的时候，Rust 编译器自动添加了调用释放资源函数的步骤。
    这种机制看似很简单了：它不过是帮助程序员在适当的地方添加了一个释放资源的函数调用而已。
    但这种简单的机制可以有效地解决一个史上最令程序员头疼的编程问题。
 */

fn main() {


    // 基本类型存储在栈中 
    /*
        所有整数类型，例如 i32 、 u32 、 i64 等。
        布尔类型 bool，值为 true 或 false 。
        所有浮点类型，f32 和 f64。
        字符类型 char。
        仅包含以上类型数据的元组（Tuples）
     */
    // "基本数据"类型的数据，不需要存储到堆中，仅在栈中的数据的"移动"方式是直接复制
    // 当变量超出范围时，Rust 自动调用释放资源函数并清理该变量的堆内存
    let x = 5;
    let _y = x;

    let s1 = String::from("hello");
    // 为了确保安全，在给 s2 赋值时 s1 已经无效了。没错，在把 s1 的值赋给 s2 以后 s1 将不可以再被使用。
    let s2 = s1;
    println!("s2 = {}", s2);


    // 克隆会被当作两个单独的资源
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 涉及函数的所有权机制
    let s = String::from("hello");
    take_onwership(s);
    // s在这里开始失效

    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    // x 的值被当作参数传入函数, 由于x为基本类型，直接在栈内copy,所以x依然有效，在这里依然可以使用 x 却不能使用 s

    let s2 = give_onwership();
    // give_onwership 移动返回值到s2

    let s3 = String::from("hello");
    // s3声明
    
    let s4 = take_and_give_back(s3);
    // s3 被移动失效，s3获得返回值所有权

    println!("s2 = {}, s4 = {}", s2, s4);


    // Reference
    // s6 此时引用的是s5, 保存s5的地址
    let s5 = String::from("hello");
    let s6 = &s5;
    println!("s5 = {}, s6 = {}", s5, s6);

    // 函数引用传递
    let s7 = String::from("hello");
    let len = calculate_length(&s7);
    println!("length of {} is {}", s7, len);


    // Reference失效
    let s8 = String::from("hello");
    let mut s9 = &s8;
    // s9租借的s8的使用权已经移动到了s10,所以需要重新租借,
    // 既然引用不具有所有权，即使它租借了所有权，它也只享有使用权（这跟租房子是一个道理）。

    let s10 = s8;
    s9 = &s10;
    println!("s9 = {}", s9);
    // 如果尝试利用租借来的权利来修改数据会被阻止：
    // s9.push_str("oob");

    // 可变引用， 当然，也存在一种可变的租借方式，就像你租一个房子，
    // 如果物业规定房主可以修改房子结构，房主在租借时也在合同中声明赋予你这种权利，你是可以重新装修房子的：
    let mut s11 = String::from("hello");
    println!("s11 = {}", s11);
    // let s12 = &mut s11;
    // s12.push_str("oob");

    // 多重可变引用是不允许的
    // Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生。
    // 由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个值被可变引用时不允许再次被任何引用。
    // let s13 = &mut s11;

    // 多重不可变引用是允许的
    let s17 = String::from("hello");
    let s14 = &s17;
    let s15 = &s17;
    println!("s17= {}, s17 = {}", &s14, s15);

    // 垂悬引用（Dangling References）
    // 如果放在有指针概念的编程语言里它就指的是那种没有实际指向一个真正能访问的数据的指针
    // 注意，不一定是空指针，还有可能是已经释放的资源）。它们就像失去悬挂物体的绳子，所以叫"垂悬引用
    // let reference_to_nothinh = dangle();

} // s2, s4被释放, s3 已经失效


// 垂悬指针出现 
// fn dangle() -> &String {
//     let s = String::from("hello");
//     return &s;
// } // s已经被释放，此时&s是一个空指针

fn take_onwership(some_string: String) {
    // 一个String参数传入，有效
    println!("{}", some_string);
} // 函数结束, 参数some_string 在这里释放


fn makes_copy(x: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", x);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn give_onwership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn take_and_give_back(a_string: String) -> String {
    return a_string;
}

fn calculate_length(s: &String) -> usize {
    // 此时s租借的是s7的使用权，指向s7的地址
    return s.len();
}


