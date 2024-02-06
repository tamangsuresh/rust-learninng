enum WebEvent {
    PageLoad,
    PageUnload,
    Keypress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}
enum Status {
    Rich,
    Poor,
}
enum Work {
    Civilian,
    Soldier,
}
enum Number {
    Zero,
    One,
    Two,
}
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;
    let status = Poor;
    let work = Civilian;
    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
