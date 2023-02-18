fn main() {
    let x = 5;

    // 函数体表达式。
    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    // 如果没有明确声明函数返回值的类型，函数将被认为是"纯过程"，不允许产生返回值
    fn five() -> i32 {
        5
    }
    println!("five = {}", five());
}