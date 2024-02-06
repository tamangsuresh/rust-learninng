#![allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}
struct Unit;
struct Pair(i32, f32);
struct Point {
    x: f32,
    y: f32,
}
struct Rectangle {
    top_left: Point,
    bottom_left: Point,
}
fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("Name {:?}", peter);

    let point = Point { x: 39.0, y: 50.0 };
    println!("Point {:?} ,{:?} ", point.x, point.y);
}
