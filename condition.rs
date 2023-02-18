fn main() {
    let n = 4;
    if n < 5 {
        println!("n < 5");
    } else {
        println!("n > 5");
    }

    // 此处m只能为bool类型
    let m = 6;
    // if m {
    //     println!("m is true");
    // }

    let w = if m > n { 4 } else {5};
    println!("w = {}", w);
}