// #[derive(Debug)]

// 引用trait模块
mod r#trait;
// 将模块中的标识符引入当前作用域
use r#trait::Description;

trait Comparable {
    // 由于需要声明 compare 函数的第二参数必须与实现该特性的类型相同，
    // 所以 Self （注意大小写）关键字就代表了当前类型（不是实例）本身。
    fn compare(&self, object: &Self)  -> i8;
}

impl Comparable for f64 {

    fn compare(&self, object: &f64) -> i8 {
        if self > object {
            1
        } else if self == object {
            0
        } else {
            -1
        }
    }
}

// 此处的范型必须使用trait
fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 0;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

fn person() -> impl r#trait::Description {
    r#trait::Person {
        name: String::from("rust"),
        age: 18
    }
}
struct B {

}

struct A {

}

impl Description for B {
    fn describe(&self) -> String {
        String::from("B")
    }
}

impl Description for A {
    fn describe(&self) -> String {
        String::from("A")
    }
}

// 特性做返回值只接受实现了该特性的对象做返回值且在同一个函数中所有可能的返回值类型必须完全一样
//  expected `A` because of return type
// fn some_function(bl: bool) -> impl Description {
//     if bl {
//         return A {};
//     } else {
//         return B {};
//     }
// }

fn main() {
    let a = [1.0,2.0,3.0,4.0,5.0,6.0];
    println!("max = {}", max(&a));

    println!("person = {}", person().describe());

    // some_function(false);
}
