fn main() {
    let condition: bool = 3 > 2;
    println!("condition = {}", condition);
    /*
    In Rust:
    And -> &&
    Or -> ||
    Not -> !
    */
    let condition: bool = (! false) && true;
    println!("condition = {}", condition);
}
