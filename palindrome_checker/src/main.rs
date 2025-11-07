use std::io;


fn main() {
    println!("Welcome to palindrome checker!");
    println!("Enter your string to check palindrome from.");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read input.");

    if is_palindrome(&clean_string(&input)) {
        println!("input is palindrome")
    } else {
        println!("input is not palindrome.")
    }
}


fn clean_string(input: &str) -> String {
    input
    .chars()
    .filter(|c| c.is_alphanumeric())
    .map(|c| c.to_lowercase().to_string())
    .collect::<String>()
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}