fn main() {
    let s = String::from("broadcast");

    // part1 中记录了字符串的起始位置和长度
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{} = {} + {}", s, part1, part2);


    let mut s1 = String::from("hello world!");

    let part3 = &s1[0..10];
    // part3.push_str("yes!");

    // 被切片引用的字符串禁止更改其值
    println!("part3 = {}", part3);

    // 凡是用双引号包括的字符串常量整体的类型性质都是 &str,也就是字符串切片
    // String 类型是 Rust 标准公共库提供的一种数据类型，
    // 它的功能更完善——它支持字符串的追加、清空等实用的操作。
    // String 和 str 除了同样拥有一个字符开始位置属性和一个字符串长度属性以外还有一个容量（capacity）属性。
    // String 和 str 都支持切片，切片的结果是 &str 类型的数据。
    let s = "hello";
    let part4 = &s[0..2];
    println!("part4 = {}", part4);

    // 将String转为str
    let s = String::from("hello");
    println!("{}", &s[..]);

    // 数组切片
    let a = [1,2,3,4,5];
    let part4 = &a[0..2];
    for i in part4.iter() {
        println!("{}", i);
    }
    // println!("part4 = {:?}", part4);
}