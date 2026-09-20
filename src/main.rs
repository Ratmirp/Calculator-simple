use calc::solve;
use std::io::{self};

fn main() {
    println!("Please write an equation");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("You typed: {input}");
    let answer = solve(input).unwrap();
    println!("{answer}");
}
