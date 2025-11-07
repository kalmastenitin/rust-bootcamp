use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome To Guessing Game!");
    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter valid number.");
                continue;
            }
        };
        println!("You guessed, {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again."),
            Ordering::Greater => println!("Too big, Try again."),
            Ordering::Equal => {
                println!("Congratulations! you guessed number!");
                break;
            }
        }
    }
}
