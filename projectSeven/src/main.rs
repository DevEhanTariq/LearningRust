fn main() {
    let mut n: u128 = 100;
    loop {
        println!("Counter: {}", n);
        n = n - 1;
        if n == 0 {
            break;
        }
    }

    println!("Hello, world!");
}
