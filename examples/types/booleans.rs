// bool: boolean type
// Use when you need to represent a simple true or false condition.

// Example 1: Checking if a user is online or not
fn example_bool1() {
    let is_online: bool = true;
    if is_online {
        println!("bool example 1: The user is online.");
    } else {
        println!("bool example 1: The user is offline.");
    }
}

// Example 2: Checking if a light is on or off
fn example_bool2() {
    let is_light_on: bool = false;
    if is_light_on {
        println!("bool example 2: The light is on.");
    } else {
        println!("bool example 2: The light is off.");
    }
}

fn main() {
    example_bool1();
    example_bool2();
}