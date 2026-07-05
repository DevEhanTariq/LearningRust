use std::io; // Crates can be at the top

fn test() {
    println!("This is a test function!");
}

fn add(a: i128, b: i128) -> i128 {
    a + b // No semi-colon means value gets returned
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input; // Can also just write return
}

fn main() {
    println!("Hello, world!");
    test();
    println!("a + b = {}", add(1, 2));

    let number = {
        let x = 3;
        x + 1 // No semi-colon means value gets returned
    };
    println!("The number is {}", number);

    println!("Hi, {}!", input().trim());
}
