

pub struct Person {
    pub name: String,
    pub age: u8
}

pub trait Description {
    fn describe(&self) -> String;
}

// 这是特性与接口的不同点：接口只能规范方法而不能定义方法，但特性可以定义方法作为默认方法，
// 因为是"默认"，所以对象既可以重新定义方法，也可以不重新定义方法使用默认的方法：
trait Name {
    fn name(&self) -> String {
        format!("[name]")
    }
}

// impl <特性名> for <所实现的类型名>
impl Description for Person {
    fn describe(&self) -> String{
        format!("{} {}", self.name, self.age)
    }
}

impl Name for Person {
    // fn name(&self) -> String{
    //     format!("{}", self.name)
    // }
}


// 特性做参数 很多情况下我们需要传递一个函数做参数，
// 例如回调函数、设置按钮事件等。在 Java 中函数必须以接口实现的类实例来传递，
// 在 Rust 中可以通过传递特性参数来实现
// fn output(object: impl Description) {
//     println!("{}", object.describe());
// }

// 这是一种风格类似泛型的语法糖，这种语法糖在有多个参数类型均是特性的情况下十分实用：
fn output<T: Description>(object: T) {
    println!("{}", object.describe()); 
}

fn output_two<T: Description>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

trait Summary {

}

trait Display {

}

// 特性作类型表示时如果涉及多个特性，可以用 + 符号表示，例如：
fn notify<T: Summary + Display>(item: T) {

}

// fn notify(item: Summary + Display) {

// }

// fn some_function<T: Display + Summary, U: Display + Description>(t: T, u: U) {

// }

fn some_function<T, U>(t: T, u: U) 
    where T: Display + Summary,
          U: Display + Description {

          }

fn main() {
    // 特性（trait）概念接近于 Java 中的接口（Interface），
    // 但两者不完全相同。特性与接口相同的地方在于它们都是一种
    // 行为规范
    //，可以用于标识哪些类有哪些方法

    // Rust 同一个类可以实现多个特性，每个 impl 块只能实现一个。

    let person = Person {
        name: String::from("john"),
        age: 10
    };
    let person1 = Person {
        name: String::from("john"),
        age: 10
    };
    let person2 = Person {
        name: String::from("john"),
        age: 10
    };
    println!("{}", person.describe());
    println!("{}", person.name());


    // 任何实现了 Descriptive 特性的对象都可以作为这个函数的参数
    // ，这个函数没必要了解传入对象有没有其他属性或方法，只需要了解它一定有 Descriptive 特性规范的方法就可以了
    output(person);

    output_two(person1, person2);
}

