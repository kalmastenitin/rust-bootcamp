use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("X Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];

    println!("Reading file: {}", file_path);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("X error reading file: {}", err);
        return;
    }

    let word_count = count_words(&contents);
    println!("Word count: {}", word_count);
}

fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}
