use std::env;

fn main() {
    // Quick example of how to accept input args using the std::env library
    let args: Vec<String> = env::args().collect();

    println!("\nRunning program: {}\n", args[0]);

    for (i, j) in args.iter().skip(1).enumerate() {
        println!("Argument #{}: {}", i, j);
    }
}
