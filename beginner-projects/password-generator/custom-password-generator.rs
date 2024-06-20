extern crate rand;
use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn main() {
    let mut rng = thread_rng();
    loop {
        println!("Select the password difficulty level:");
        println!("1. Easy");
        println!("2. Intermediate");
        println!("3. Hard");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let (characters, password_length) = match choice {
            1 => ("abcdefghijklmnopqrstuvwxyz", 8),
            2 => ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ", 12),
            3 => ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()", 16),
            4 => break,
            _ => continue,
        };

        let password: String = (0..password_length).map(|_| *rng.choose(&characters.chars().collect::<Vec<char>>()).unwrap()).collect();
        println!("Your password is: {}", password);
    }
}