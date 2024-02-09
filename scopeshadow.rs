fn main() {
    let n = 1;
    let v = 'v';
    {
        let m = 3;
        println!("Hello latest number is {}", m);
        println!("Varible {}", v);
        let v = "asdf";
        println!("Variable {}", v);
    }
    println!("Number is {}", n);
    println!("Variable is {}", v);
}
