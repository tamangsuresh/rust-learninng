fn main() {
    let x = 5;
    let y = x;
    let string = String::from("Hello");
    // let str = string.clone();
    let (len, s) = cal_len(string);
    // println!("Hello, world, {x} , {y}");
    // println!("{string} {str}");
    println!("{len} {s}")
}
fn cal_len(str: String) -> (usize, String) {
    let length = str.len();
    return (length, str);
}
