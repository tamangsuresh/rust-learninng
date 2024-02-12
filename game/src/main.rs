use std::io;

fn main() {
    println!("Guess the number: ");

    println!("Please the number: ");

    let mut guess_number = String::new();
    io::stdin()
        .read_line(&mut guess_number)
        .expect("Fail to read the lie");

    println!("The guess number is {guess_number}",);
}
