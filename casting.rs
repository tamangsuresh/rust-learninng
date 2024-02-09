// Suppress all warnings from casts which overflow.
#![allow(overflowing_literals)]
fn main() {
    let decimal = 58.3533;
    let integer = 100;
    let char = "E";
    println!(
        "Decimal -> integer -> char {} {}  {}",
        decimal, integer, char
    );
}
