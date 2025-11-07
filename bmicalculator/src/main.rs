use core::f64;
use std::io::{self};

fn main() {
    println!("Bmi Calculator using Age, Weight, Height.");

    println!("Enter your age: ");
    let _age = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for age. Please enter valid number");
            return;
        }
    };

    println!("Enter your height: ");
    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for height. Please enter valid number for height. ");
            return;
        }
    };

    println!("Enter your weight: ");
    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("Invalid input for weight. please enter valid number for weight. ");
            return;
        }
    };

    let bmi = weight / height.powf(2.0);
    println!("your bmi is :{}",bmi*10000.0);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read lines.");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None
    }
}