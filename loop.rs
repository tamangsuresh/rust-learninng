fn main() {
    'first: loop {
        println!("First Loop");
        'second: loop {
            println!("Second Loop");
            'third: loop {
                println!("Third loop");
                'fourth: loop {
                    println!("Fourth loop");
                    break 'fourth;
                }
                break 'third;
            }
            break 'second;
        }
        break 'first;
    }
}
