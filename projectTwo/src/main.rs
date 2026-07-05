fn main() {
    println!("Hello, world!");

    println!();

    let mut i: i8 = 127; // i is integer, 8 is 8 bits.  *Note, this includes a negative 8th bit, this means -128 -> 127
    println!("i = {}", i);
    i = -128;
    println!("i = {}", i);
    /*
    All integer types:
    i8
    i16
    i32
    i64
    i128
    */

    println!();

    let mut u: u8 = 255; // i is unsigned integer, meaning no negative 8th bit
    println!("u = {}", u);
    u = 0;
    println!("u = {}", u);
    /*
    All UInt types:
    u8
    u16
    u32
    u64
    u128
    */

    println!();

    let f: f32 = 9.8; // f is floating point
    println!("f = {}", f);
    /*
    All float types:
    f32
    f64
    */

    println!();

    let mut b: bool = true; // boolean, true or false
    println!("b = {}", b);
    b = false;
    println!("b = {}", b);
    /*
    All boolean types:
    true
    false
    */

    println!();

    let mut c: char = 'a'; // single character in single quotes
    println!("c = {}", c);
    c = '@';
    println!("c = {}", c);
    /*
    Any single character on your keyboard
    */

    println!();


    let mut tup: (i8, f32, bool, char) = (12, 9.8, false, '?');
    println!("tup = {}", tup.0); // .0 is referencing an index
    println!("tup = {}", tup.1);
    println!("tup = {}", tup.2);

    tup.0 = 5;
    println!("tup = {}", tup.0);

    println!("tup = {:?}", tup); // Prints the Tuple

    println!();

    let mut arr: [i8; 5] = [1, 2, 3, 4, 5]; // [i8; 5], is the type, and the length
    println!("arr = {}", arr[0]);
    println!("arr = {:?}", arr);
    arr[0] = 12;
    println!("arr = {}", arr[0]);

    println!();

    let x: u8 = 4;
    let y = x; // y is type u8
    println!("x = {}, y = {}", x, y);

    println!();

    let text: &str = "Hi"; // string
    println!("{}", text);
}
