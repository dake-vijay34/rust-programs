9. Tokenize and Iterate over a String
fn main() {
    let sentence = "Rust is fast and memory-safe";
    for word in sentence.split_whitespace() {
        println!("{}", word);
    }
}
