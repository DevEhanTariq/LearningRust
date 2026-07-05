fn main() {
    let x: i8 = 5;
    let y: i8 = 3;

    let sum = x + y;
    println!("sum = {}", sum);

    let sum = y - x;
    println!("subtraction = {}", sum);

    let x: i8 = 2;
    let y: i8 = 6;

    let sum = x * y;
    println!("multiplication = {}", sum);

    let sum = x / y; // Of type i8, meaning no floating point value
    println!("division = {}", sum);

    let x: f32 = 2.0;
    let y: f32 = 6.0;

    let sum = x / y;
    println!("division = {}", sum);

    let x: f32 = 255.0;
    let y: f32 = 10.0;

    let sum = x % y; // Mod gives the remainder
    println!("mod = {}", sum);

    let x: i8 = 5; // Different type to y
    let y: u8 = 3;

    let sum = (x as u8) % y; // 'as' changes its type
    println!("sum = {}", sum);

    use std::io;

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to read line");

    let a: i128 = a.trim().parse().unwrap(); // '.parse()', gives an integer if it is able to give an integer. '.unwrap()' returns the actual integer type.
    println!("a = {}", a + 2);
}
