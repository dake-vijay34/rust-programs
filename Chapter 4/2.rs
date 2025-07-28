2. Function test_divisibility_by_3_4
fn test_divisibility_by_3_4(num: i32) -> i32 {
    if num % 3 == 0 && num % 4 == 0 {
        0
    } else if num % 3 == 0 {
        1
    } else if num % 4 == 0 {
        2
    } else {
        -1
    }
}

fn main() {
    let num = 12;
    println!("Result: {}", test_divisibility_by_3_4(num));
}
