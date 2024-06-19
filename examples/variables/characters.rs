// char: character type
// Use when you need to represent a single Unicode character.

// Example 1: Representing a letter in the alphabet
fn example_char1() {
    let letter: char = 'a';
    println!("char example 1: The letter is {}.", letter);
}

// Example 2: Representing a digit
fn example_char2() {
    let digit: char = '9';
    println!("char example 2: The digit is {}.", digit);
}

// Example 3: Representing a special character
fn example_char3() {
    let special_character: char = '#';
    println!("char example 3: The special character is {}.", special_character);
}

// Example 4: Representing an emoji
fn example_char4() {
    let emoji: char = '😀';
    println!("char example 4: The emoji is {}.", emoji);
}

fn main() {
    example_char1();
    example_char2();
    example_char3();
    example_char4();
}