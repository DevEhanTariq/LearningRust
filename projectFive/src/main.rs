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

    let month: &str = "February";

    if month == "January" {
        println!("The month is January!");
    } else if month == "February" {
        println!("The month is February!");
    } else {
        println!("The month is not January or February!");
    }
}
