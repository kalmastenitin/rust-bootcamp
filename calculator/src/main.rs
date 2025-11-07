use core::f64;
use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /");
    println!("Enter your expression (e.g., 5 + 3):");

    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    if tokens.len() != 3 {
        println!("X invalid Input. Please follow the expression format (e.g., 5 + 3)");
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("X invaild first number");
            return
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("X invaild second number");
            return
        }
    };

    let result = match operator {
        "+" => add(num1 , num2),
        "-" => substract(num1 ,num2),
        "*" => multiply(num1 , num2),
        "/" => divide(num1 , num2),
        _ => {
            println!("X invalid operator. Use +, -, *, or /.");
            return
        }
    };
    println!("Result: {:.2}", result);
}


fn add(a: f64, b: f64) -> f64 {
    a+b
}

fn substract(a: f64, b: f64) -> f64 {
    a-b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64,b: f64) -> f64 {
    a / b
}