1. Program to Find the Factorial using Functions
fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

fn main() {
    let num = 5;
    println!("Factorial of {} is {}", num, factorial(num));
}
