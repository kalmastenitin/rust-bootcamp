use std::io;

fn main() {
    println!("Welcome to fibonacci number generator.");
    println!("Enter a number of terms you want to generate: ");

    let n_terms = match get_input_as_u32() {
        Some(Value) => Value,
        None => {
            println!("invalid input. enter only number");
            return;
        }
    };

    let sequence = generate_fibonacci(n_terms);
    println!("✅ Fibonacci Sequence ({} terms): {:?}",n_terms, sequence);
}


fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .expect("❌ unable to read input.");

    match input.trim().parse::<u32>() {
        Ok(Value) => Some(Value),
        Err(_) => None
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0)
    }
    if n >= 2 {
        sequence.push(1)
    }
    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next)
    }
    sequence
}