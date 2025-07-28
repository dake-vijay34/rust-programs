8. String Manipulation using Core Methods
fn main() {
    let mut s = String::from("   Hello Rust Programming   ");

    println!("Capacity: {}", s.capacity());
    println!("Contains 'Rust'? {}", s.contains("Rust"));

    let replaced = s.replace("Rust", "World");
    println!("Replaced String: {}", replaced);

    let trimmed = s.trim();
    println!("Trimmed String: '{}'", trimmed);
}
