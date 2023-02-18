// 基本类型 int, float, bool, char
fn main() {

    // let i: i64 = 10;
    // let f: f64 = 3.2;
    // let b: bool = true;
    // let c: char = 'c';

    // 复合类型
    let tup: (u32, f64, bool, char) = (1, 3.2, true, 'c');
    let (x, y, z, w) = tup;
    println!("x = {0}, y = {1}, z = {2}, w = {3}", x, y, z, w);

    // 数组
    let a = [1, 2, 3];
    let b = ['a','b', 'c'];
    let c: [i32; 3] = [4, 5, 6];
    let d = [3; 5];

    let mut e = [3, 4];
    e[0] = 5;
    println!("a = {:?}, b = {:?}, c = {:?}, d = {:?}, e = {:?}", a, b, c, d, e); 
}