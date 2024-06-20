// f32: single-precision float
// Use when you need to represent the weight of a person, which can be a decimal.
fn example_f32() {
    let weight: f32 = 65.5;
    println!("f32 example: My weight is {} kg.", weight);
}

// f64: double-precision float
// Use when you need to represent the distance between two points on Earth, which can be a decimal.
fn example_f64() {
    let distance: f64 = 3844.7;
    println!("f64 example: The distance to the destination is {} km.", distance);
}

fn main() {
    example_f32();
    example_f64();
}