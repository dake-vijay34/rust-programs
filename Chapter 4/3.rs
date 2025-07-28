3. Pass by Value and Pass by Reference
fn pass_by_value(mut x: i32) {
    x += 5;
    println!("Inside pass_by_value: {}", x);
}

fn pass_by_reference(x: &mut i32) {
    *x += 5;
    println!("Inside pass_by_reference: {}", x);
}

fn main() {
    let mut a = 10;
    pass_by_value(a);
    println!("After pass_by_value: {}", a);

    pass_by_reference(&mut a);
    println!("After pass_by_reference: {}", a);
}
