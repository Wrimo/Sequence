mod types;
mod parser;
use crate::types::Production;
use parser::{get_productions, parse};


fn main() {
    let productions: Vec<Production> = get_productions();
    let test: &str = "number";
    let test1: &str = "number + number - number";
    let test2: &str = "number - number + number - number";
    println!("{} -> {}", test, parse(&productions, test));
    println!("\n\n");
    println!("{} -> {}", test1, parse(&productions, test1));
    println!("\n\n");
    println!("{} -> {}", test2, parse(&productions, test2));
}
