// String: string type
// Use when you need to represent a sequence of characters.

// Example 1: Creating a string
fn example_string1() {
    let greeting: String = String::from("Hello, world!");
    println!("String example 1: {}", greeting);
}

// Example 2: Concatenating strings
fn example_string2() {
    let mut greeting: String = String::from("Hello");
    greeting.push_str(", world!"); // push_str() appends a string slice to the String
    println!("String example 2: {}", greeting);
}

// Example 3: String slicing
fn example_string3() {
    let greeting: String = String::from("Hello, world!");
    let hello = &greeting[0..5]; // This is a string slice that contains the first 5 characters of the string
    println!("String example 3: {}", hello);
}

// Example 4: Converting to uppercase
fn example_string4() {
    let greeting: String = String::from("Hello, world!");
    let uppercased = greeting.to_uppercase(); // This creates a new String that contains the uppercase version of the original String
    println!("String example 4: {}", uppercased);
}

// Example 5: Sorting a string
fn example_string5() {
    let mut chars: Vec<char> = "hello".chars().collect();
    chars.sort();
    let sorted_string: String = chars.into_iter().collect();
    println!("String example 5: {}", sorted_string);
}

fn main() {
    example_string1();
    example_string2();
    example_string3();
    example_string4();
    example_string5();
}