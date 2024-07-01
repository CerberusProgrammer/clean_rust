use std::io::{self, Write};

fn main() {
    loop {
        println!("Select the type of conversion:");
        println!("1. Length");
        println!("2. Weight");
        println!("3. Temperature");
        println!("4. Exit");

        let choice = read_input().trim().to_string();

        match choice.as_str() {
            "1" => length_conversion(),
            "2" => weight_conversion(),
            "3" => temperature_conversion(),
            "4" => break,
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn length_conversion() {
    println!("Select the length conversion:");
    println!("1. Kilometers to Miles");
    println!("2. Miles to Kilometers");

    let choice = read_input().trim().to_string();

    match choice.as_str() {
        "1" => {
            println!("Enter kilometers:");
            let km: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{} kilometers is {} miles", km, km * 0.621371);
        },
        "2" => {
            println!("Enter miles:");
            let miles: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{} miles is {} kilometers", miles, miles / 0.621371);
        },
        _ => println!("Invalid option, please try again."),
    }
}

fn weight_conversion() {
    println!("Select the weight conversion:");
    println!("1. Kilograms to Pounds");
    println!("2. Pounds to Kilograms");

    let choice = read_input().trim().to_string();

    match choice.as_str() {
        "1" => {
            println!("Enter kilograms:");
            let kg: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{} kilograms is {} pounds", kg, kg * 2.20462);
        },
        "2" => {
            println!("Enter pounds:");
            let pounds: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{} pounds is {} kilograms", pounds, pounds / 2.20462);
        },
        _ => println!("Invalid option, please try again."),
    }
}

fn temperature_conversion() {
    println!("Select the temperature conversion:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let choice = read_input().trim().to_string();

    match choice.as_str() {
        "1" => {
            println!("Enter Celsius:");
            let celsius: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{}°C is {}°F", celsius, celsius * 1.8 + 32.0);
        },
        "2" => {
            println!("Enter Fahrenheit:");
            let fahrenheit: f64 = read_input().trim().parse().expect("Please enter a valid number");
            println!("{}°F is {}°C", fahrenheit, (fahrenheit - 32.0) / 1.8);
        },
        _ => println!("Invalid option, please try again."),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}