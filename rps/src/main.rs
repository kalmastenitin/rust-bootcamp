use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Rock, Paper, Sissor's Game!");
    println!("Instructions: Enter 'rock', 'paper' or 'sissors'. Type 'quit' to exit the game.",);

    loop {
        println!("\n🪨 📃 ✂️ Make Your Choice: ");

        let choice = get_user_input();
        
        if choice == "quit" {
            println!("Thanks for playing. Good Bye! 👋");
            break;
        }

        let cc = get_computer_choice(); // take computer choise
        println!("🤖 computer choise: {}", cc);

        match determine_winner(&choice, &cc) {
            GameResult::Win => println!("🏆 You win!"),
            GameResult::Lose => println!("☺️ You lose!"),
            GameResult::Draw => println!("🤝 I's a draw!"),
        }
    }
}


fn get_user_input() -> String {
    let mut choice = String::new();
    io::stdin()
    .read_line(&mut choice)
    .expect("❌ failed to parse user input.");

    let choice = choice.trim().to_lowercase();

    match choice.as_str(){
        "rock" | "paper" | "scissor" | "quit" => choice,
        _ => {
            println!("❌ Invalid choice. Please enter rock paper scissors and quit only.");
            get_user_input()
        }
    }
}

fn get_computer_choice() -> String{
    let choice = ["rock", "paper", "scissor"];
    let index = rand::rng().random_range(0..choice.len());
    choice[index].to_string()
}

enum GameResult {
    Win,
    Lose,
    Draw
}

fn determine_winner(user: &str, cc: &str) -> GameResult{
    match (user, cc) {
        ("rock", "scissor") => GameResult::Win,
        ("paper","rock") => GameResult::Win,
        ("scissor","paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose,
    }
}