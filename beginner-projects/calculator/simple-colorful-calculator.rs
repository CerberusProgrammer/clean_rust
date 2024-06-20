use std::io::{self, Write};
use colored::*;

fn main() {
    loop {
        println!("{}", "Please select an operation:".green().bold());
        println!("{} Add", "1)".blue());
        println!("{} Subtract", "2)".yellow());
        println!("{} Multiply", "3)".magenta());
        println!("{} Divide", "4)".cyan());
        println!("{} Exit", "5)".red());

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 5 {
            break;
        }

        print!("{} ", "Enter the first number:".green().bold());
        io::stdout().flush().unwrap();
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).unwrap();
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("{} ", "Enter the second number:".green().bold());
        io::stdout().flush().unwrap();
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).unwrap();
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = match choice {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 == 0.0 {
                    println!("{}", "Error: Division by zero is not allowed.".red().bold());
                    continue;
                } else {
                    num1 / num2
                }
            }
            _ => continue,
        };

        println!("{} {}", "The result is:".green().bold(), result);
    }
}