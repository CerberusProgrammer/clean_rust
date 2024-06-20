extern crate rand;
use rand::{thread_rng, Rng};

fn main() {
    let password_length = 12;
    let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()";
    let mut rng = thread_rng();
    let password: String = (0..password_length).map(|_| *rng.choose(&characters.chars().collect::<Vec<char>>()).unwrap()).collect();
    println!("{}", password);
}