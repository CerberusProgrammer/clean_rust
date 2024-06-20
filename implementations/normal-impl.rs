// Step 1: Define the struct
// Here we're defining a struct called `Person` that has several fields.
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    city: String,
    country: String,
    location: (f64, f64), // latitude, longitude
}

// Step 2: Implement methods for the struct
impl Person {
    // The `new` method takes several arguments and returns a new `Person`.
    fn new(first_name: String, last_name: String, age: u8, city: String, country: String, location: (f64, f64)) -> Person {
        Person { first_name, last_name, age, city, country, location }
    }

    // The `full_name` method returns the full name of the person.
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // The `greet` method prints a greeting.
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old. I live in {}, {}.", self.full_name(), self.age, self.city, self.country);
    }

    // The `move_to` method changes the person's city, country, and location.
    fn move_to(&mut self, city: String, country: String, location: (f64, f64)) {
        self.city = city;
        self.country = country;
        self.location = location;
    }
}

// Step 3: Use the struct and its methods
fn main() {
    // We create a new `Person` named `alice` using the `new` method.
    let mut alice = Person::new(String::from("Alice"), String::from("Smith"), 30, String::from("New York"), String::from("USA"), (40.7128, -74.0060));

    // We call the `greet` method on `alice`.
    alice.greet();

    // We change `alice`'s city, country, and location using the `move_to` method.
    alice.move_to(String::from("Los Angeles"), String::from("USA"), (34.0522, -118.2437));

    // We call the `greet` method on `alice` again.
    alice.greet();
}