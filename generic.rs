#[derive(Debug)]

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    // 这里添加&的原因是T为范型， move occurs because `self.x` has type `T`
    // which does not implement the `Copy` trait 所以无法通过编译时检查
    fn x(&self) -> &T {
        &self.x
    }
}

// impl Point<f64> {
//     fn x(&self) -> f64 {
//         self.x
//     }
// }


fn main() {
    // 泛型机制是编程语言用于表达类型抽象的机制，一般用于功能确定、数据类型待定的类，如链表、映射表等。
    let a = [2,3,4,5,6];
    println!("max = {}", max(&a));

    // 结构体和枚举中的范型
    let p1 = Point{x: 1, y:2};
    let p2 = Point{x: 1.0, y: 2.0};
    println!("p1 = {:?}, p2 = {:?}", p1, p2);

    let p3 = Point{x: 1, y: 2};
    println!("p.x = {}", p3.x());
}

fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 0;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    return array[max_index];
}

// fn max<T>(array: &[T]) -> T {
//     let mut max_index = 0;
//     let mut i = 0;
//     while i < array.len() {
//         if array[i] > array[max_index] {
//             max_index = i;
//         }
//         i += 1;
//     }
//     return array[max_index];
// }

