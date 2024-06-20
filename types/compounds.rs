// Tuple: compound type that groups together values of different types
// Use when you need to group together a few values and you don't expect the group to grow or shrink.

// Example 1: Representing a 2D point
fn example_tuple1() {
    let point: (i32, i32) = (10, 20);
    println!("Tuple example 1: The point is at ({}, {}).", point.0, point.1);
}

// Example 2: Returning multiple values from a function
fn example_tuple2() -> (i32, i32) {
    (30, 40)
}

// Example 3: Swapping values in a tuple
fn example_tuple3() {
    let mut point = (1, 2);
    println!("Tuple example 3: Before swap: {:?}", point);
    std::mem::swap(&mut point.0, &mut point.1);
    println!("Tuple example 3: After swap: {:?}", point);
}

// Array: compound type that groups together multiple values of the same type
// Use when you need to group together many values and you know the group won't grow or shrink.

// Example 4: Representing a list of scores
fn example_array1() {
    let scores: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Array example 1: The first score is {}.", scores[0]);
}

// Example 5: Initializing an array with the same value
fn example_array2() {
    let scores: [i32; 5] = [0; 5];
    println!("Array example 2: The first score is {}.", scores[0]);
}

// Example 6: Modifying an element in an array
fn example_array3() {
    let mut scores = [10, 20, 30, 40, 50];
    println!("Array example 3: Before modification: {:?}", scores);
    scores[0] = 60;
    println!("Array example 3: After modification: {:?}", scores);
}

fn main() {
    example_tuple1();
    let point = example_tuple2();
    println!("Tuple example 2: The point is at ({}, {}).", point.0, point.1);
    example_tuple3();
    example_array1();
    example_array2();
    example_array3();
}