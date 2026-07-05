use std::io; // Gets the 'io' module from the 'std' crate/library (standard library)
// :: is a path separator, like random.randint() in python with the .

fn main() {
    println!("Hello, world!");

    println!();

    let mut name = String::new(); // String::new() creates an empty string object
    io::stdin()
        .read_line(&mut name) // If no &, it would be a copy, & makes it a reference, mut makes it mutable
        .expect("failed to read line"); // 'expect' checks if read_line gave a valid value

    let name = name.trim(); // Trims the spaces/new line characters at the end of a string

    println!("Hello, {}!", name);
}
