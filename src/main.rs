use std::io;
use calculator::{calculate, Operation};

fn main() {
    println!("Enter first number: ");
    let num1 = read_number().parse::<f64>().expect("Please enter a valid number.");

    println!("Enter operator: ");
    let operator = read_operator();
    let operation = match operator.as_str() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operator.");
            return;
        }
    };
    println!("Enter second number:");
    let num2 = read_number().parse::<f64>().expect("Please enter a valid number");

    match calculate(operation, num1, num2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero"),
    }
}

fn read_number() -> String {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    number.trim().to_string()
}

fn read_operator() -> String {
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line.");
    operator.trim().to_string()
}
