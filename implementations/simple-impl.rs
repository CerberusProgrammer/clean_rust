// Step 1: Define the struct
// Here we're defining a struct called `Person` that has two fields: `name` and `age`.
struct Person {
    name: String,
    age: u8,
}

// Step 2: Implement methods for the struct
// Here we're defining a `new` method that creates a new `Person`.
// We're also defining a `greet` method that prints a greeting.
impl Person {
    // The `new` method takes two arguments and returns a new `Person`.
    // This method is called a "constructor" in other languages.
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // The `greet` method takes a reference to `self`, which represents the current instance of `Person`.
    // This method prints a greeting that includes the person's name and age.
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}

// Step 3: Use the struct and its methods
fn main() {
    // We create a new `Person` named `alice` using the `new` method.
    let alice = Person::new(String::from("Alice"), 30);

    // We call the `greet` method on `alice`.
    alice.greet();
}