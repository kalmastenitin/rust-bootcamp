use std::io;

fn main() {
    println!("Welcome to prime checker!");
    println!("Enter a number to check is it prime or not: ");
    let number = match get_input_as_u32(){
        Some(value) => value,
        None => {
            println!("❌ Invalid input, please enter a number.");
            return;
        }
    };
    if number <= 1 {
        println!("❌ number mus t be greater than 1.");
        return;
    }

    if is_prime(number) {
        println!("number is prime")
    } else {
        println!("number is not prime.")
    }
    
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true; // 2 is only even prime number
    }

    if n % 2 == 0 {
        return false; // eliminate even number greater than 2;
    }

    let limit = (n as f64).sqrt() as u32 + 1;
    println!("limit: {:?}",(n as f64).sqrt());
    for i in 3..limit{
        if n % i == 0 {
            return false;
        }
    }
    true


}


fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("❌ unable to read line.");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None
    }
}