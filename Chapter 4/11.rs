11. Write a program to find all words starting with a “c” in a string passed as a parameter.
Concatenate them together and return the result.

fn find_words_starting_with_c(input: &str) -> String {
    input
        .split_whitespace()
        .filter(|word| word.to_lowercase().starts_with('c'))
        .collect::<Vec<&str>>()
        .join("")
}

fn main() {
    let text = "Rust is cool, calm and collected code that compiles cleanly.";
    let result = find_words_starting_with_c(text);
    println!("Concatenated words starting with 'c': {}", result);
}
