#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let area = Rectangle {
        width: 50,
        height: 100,
    };
    let tuple = (50, 100);
    println!("Area {:?}", area.area());
    println!("Tuple {:?}", tuple);
}
fn calculate(s: &mut String) -> usize {
    s.push_str("s,What the check brow");
    return s.len();
}
