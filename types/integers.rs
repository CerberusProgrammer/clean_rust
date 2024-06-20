// i8: signed 8-bit integer, range: -128 to 127
// Use when you need to represent the temperature in Celsius, which can be negative.
fn example_i8() {
    let temperature_celsius: i8 = -10;
    println!("i8 example: It's {} degrees Celsius outside.", temperature_celsius);
}

// u8: unsigned 8-bit integer, range: 0 to 255
// Use when you need to represent the age of a person, which can't be negative.
fn example_u8() {
    let age: u8 = 30;
    println!("u8 example: I am {} years old.", age);
}

// i16: signed 16-bit integer, range: -32768 to 32767
// Use when you need to represent the population of a small town, which can't be negative.
fn example_i16() {
    let population: i16 = 5000;
    println!("i16 example: The population of the town is {}.", population);
}

// u16: unsigned 16-bit integer, range: 0 to 65535
// Use when you need to represent the ID of a product, which can't be negative.
fn example_u16() {
    let product_id: u16 = 12345;
    println!("u16 example: The product ID is {}.", product_id);
}

// i32: signed 32-bit integer, range: -2147483648 to 2147483647
// Use when you need to represent the balance of a bank account, which can be negative.
fn example_i32() {
    let balance: i32 = -500;
    println!("i32 example: The balance of the bank account is {}.", balance);
}

// u32: unsigned 32-bit integer, range: 0 to 4294967295
// Use when you need to represent the number of views of a YouTube video, which can't be negative.
fn example_u32() {
    let views: u32 = 1000000000;
    println!("u32 example: The YouTube video has {} views.", views);
}

// i64: signed 64-bit integer, range: -9223372036854775808 to 9223372036854775807
// Use when you need to represent the number of milliseconds since the Unix epoch, which can be negative.
fn example_i64() {
    let timestamp: i64 = 1615890123000;
    println!("i64 example: The timestamp is {}.", timestamp);
}

// u64: unsigned 64-bit integer, range: 0 to 18446744073709551615
// Use when you need to represent the total number of cells in a large spreadsheet, which can't be negative.
fn example_u64() {
    let cells: u64 = 1000000000000;
    println!("u64 example: The spreadsheet has {} cells.", cells);
}

// i128: signed 128-bit integer, range: -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
// Use when you need to represent extremely large numbers, such as the number of atoms in the universe.
fn example_i128() {
    let atoms: i128 = 100000000000000000000000000000000;
    println!("i128 example: There are approximately {} atoms in the universe.", atoms);
}

// u128: unsigned 128-bit integer, range: 0 to 340282366920938463463374607431768211455
// Use when you need to represent extremely large numbers that can't be negative, such as the number of possible UUIDs.
fn example_u128() {
    let uuids: u128 = 340282366920938463463374607431768211455;
    println!("u128 example: There are {} possible UUIDs.", uuids);
}

// isize: signed pointer-sized integer. Its size depends on the machine architecture (32-bit or 64-bit)
// Use when you need to represent the difference between two memory addresses.
fn example_isize() {
    let diff: isize = -500;
    println!("isize example: The difference between the two memory addresses is {}.", diff);
}

// usize: unsigned pointer-sized integer. Its size depends on the machine architecture (32-bit or 64-bit)
// Use when you need to represent the size of a memory block or an index into an array.
fn example_usize() {
    let size: usize = 1000;
    println!("usize example: The size of the memory block is {}.", size);
}

fn main() {
    example_i8();
    example_u8();
    example_i16();
    example_u16();
    example_i32();
    example_u32();
    example_i64();
    example_u64();
    example_i128();
    example_u128();
    example_isize();
    example_usize();
}