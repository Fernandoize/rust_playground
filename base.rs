fn main() {
    // 可变变量与不可变变量 添加mut，不可变变量不可赋值修改
    // let mut a = 123;
    // // // a = "abc";
    // // // a = 4.56; 
    // a = 456;

    // const a: i32 = 123;
    // let a = 456;

    // shadow 重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。但可变变量赋值仅能发生值的变化。
    let x: i32 = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}