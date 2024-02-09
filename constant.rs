static NAME: &str = "Suresh Tamang";
const ADDRESS: &str = "Kathmandu";
static LANGUAGE: &str = "NP";
const price: i32 = 10;
fn is_big(n: i32) -> bool {
    return n > price;
}
fn main() {
    println!("My name {} ", NAME);
    println!("And my address is {}", ADDRESS);
    println!("Is the number greater than ? {}", is_big(5));
}
