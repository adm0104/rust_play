use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Andrew Malone")]
struct Opts {
    #[clap(short = 'a', long)]
    a: i32,

    #[clap(short = 'b', long)]
    b: i32,

    #[clap(short = 'o', long = "op")]
    op: String
}

trait Operations {
    fn add(&self) -> i32;
    fn subtract(&self) -> i32;
    fn divide(&self) -> i32;
    fn multiply(&self) -> i32;
}

struct Calculator {
    a: i32,
    b: i32
}

impl Operations for Calculator {
    fn add(&self) -> i32 {
        return self.a + self.b;
    }
    fn subtract(&self) -> i32 {
        return self.a - self.b;
    }
    fn divide(&self) -> i32 {
        return self.a / self.b;
    }
    fn multiply(&self) -> i32 {
        return self.a * self.b;
    }
}

fn main() { 
    let opts: Opts = Opts::parse();

    let a: i32 = opts.a;
    let b: i32 = opts.b;
    let op: &str = &opts.op;

    let allowed_ops = vec!["add", "sub", "mul", "div"];
    if !allowed_ops.contains(&op) {
        println!("Please specify a valid operation")
    }
    else {
        let calculator: Calculator = Calculator {a: a, b: b};
        let result: i32;
        let sym: char;
        match op {
            "add" => {
                result = calculator.add();
                sym = '+';
            }
            "sub" => {
                result = calculator.subtract();
                sym = '-';
            }
            "mul" => {
                result = calculator.multiply();
                sym = '*';
            }
            "div" => {
                result = calculator.divide();
                sym = '/';
            }
            _ => unreachable!(),
        }
        println!("{} {} {} = {}",a,sym,b,result);
    }
}
