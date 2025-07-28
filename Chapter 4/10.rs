10. Push a String into a String
fn main() {
    let mut s1 = String::from("Hello");
    let s2 = " World";
    s1.push_str(s2);
    println!("{}", s1);
}
