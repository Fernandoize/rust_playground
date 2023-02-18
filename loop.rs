fn main() {
    // while循环
    let mut n = 1;
    while n != 4 {
        println!("n = {}", n);
        n += 1;
    }

    // for循环 a.iter() 代表 a 的迭代器（iterator）
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("i = {}", i);
    }

    for i in 6..10 {
        println!("i = {}", i);
    }

    // 在循环体内选择是否继续循环
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}