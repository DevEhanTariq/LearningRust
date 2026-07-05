fn main() {
    println!("Hello, world!"); // Prints on a new line

    println!();

    let mut x = 4; // Assigns 4 to x, and is mutable
    println!("The value of x is: {}", x); // The {} is where x will go
    x = x + 1; // Gets changed because it's mutable
    println!("The value of x is now: {}", x);

    println!();

    let y = 5;
    println!("The value of y is: {}", y);
    let y = "Hello"; // Redefines the variable, allowing for type change
    println!("The value of y is: {}", y);

    println!();

    const GRAVITY: f64 = 9.8; // Constants can never be changed, and need a type
    println!("The value of gravity is: {}", GRAVITY);
}
