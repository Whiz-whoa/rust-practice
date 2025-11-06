use std::env;

fn main() {
    let args = env::args().collect();
    println!("Program: {}", args[0]);
    if args.len() > 1{
        println!("First argument: {}", args[1])
    }
}