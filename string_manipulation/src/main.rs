use std::io::{self, Write};

fn main() {
    println!("string manipulation tool");

    loop {
        println!("\nChoose operation:");
        println!("1. Reverse");
        println!("2. Uppercase");
        println!("3. Lowercase");
        println!("4. Trim");
        println!("5. Find substring");
        println!("6. Replace Text");
        println!("7. Exit");

        let choice = prompt("Enter your choice:");
        match choice.trim(){
            "1" => {
                let s = prompt("Enter a string: ");
                println!("🔄 Reversed: {}", s.chars().rev().collect::<String>());
            }
            "2" => {
                let s = prompt("Enter a string: ");
                println!("🔠 Uppercase: {}", s.to_uppercase());
            }
            "3" => {
                let s = prompt("Enter a string: ");
                println!("🔡 Lowercase: {}", s.to_lowercase());
            }
            "4" => {
                let s = prompt("Enter a string: ");
                println!("✂️ Trimmed: {}",s.trim());
            }
            "5" => {
                let s = prompt("Enter the main string: ");
                let sub = prompt("Enter substring to find: ");
                if s.contains(&sub) {
                    println!("✅ Substring found '{}' ",sub);
                } else {
                    println!("❌ Substring Not found.");
                }
            }
            "6" => {
                let s = prompt("Enter the main string: ");
                let old = prompt("Text to replace: ");
                let new = prompt("Replace with: ");

                println!("🔄 Result: {}", s.replace(&old, &new));
            }
            "7" => {
                println!("Good Bye! 👋");
                break;
            }
            _ => println!("❌ Invalid Choice."),
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}",message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}