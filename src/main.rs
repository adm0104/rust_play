use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let something: Vec<String> = env::args().collect();
    for (index, argument) in args.iter().enumerate().skip(1) {
        println!("Argument {}: {}", index, argument);
    }
    for (index, argument) in something.iter().enumerate().skip(1) {
        println!("Argument {}: {}", index, argument);
    }
}
