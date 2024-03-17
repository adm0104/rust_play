use clap::Parser;

/// My toy project for learning to use clap
#[derive(Parser)]
#[clap(version = "1.0", author = "Andrew Malone")]
struct Opts {
    #[clap(short = 'a', long = "a_variable")]
    a: f64,

    #[clap(short = 'b', long = "b_variable")]
    b: f64,

    #[clap(short = 'v', long = "verbose")]
    verbose: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let a: f64 = opts.a;
    let b: f64 = opts.b;

    println!("a: {}", a);
    println!("b: {}", b);
    println!("a + b = {}", add(a,b));

}

fn add(a: f64, b: f64) -> f64 {
    return a + b;
}