fn main() {
    fizz(100);
}
fn fizz(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
fn fizzbuzz(n: u32) {
    if is_divisible(n, 15) {
        println!("Fi")
    } else if is_divisible(n, 3) {
        println!("EEE");
    } else if is_divisible(n, 5) {
        println!("LLL");
    } else {
        println!("{}", n);
    }
}
fn is_divisible(a: u32, b: u32) -> bool {
    if b == 0 {
        return false;
    }
    a % b == 0
}
