mod types;
mod parser;
use parser::parse;


fn main() {
    let test: &str = "number";
    let test1: &str = "number + number - number";
    let test2: &str = "number - number + number - number";
    println!("{} -> {}", test, parse(test));
    println!("\n\n");
    println!("{} -> {}", test1, parse(test1));
    println!("\n\n");
    println!("{} -> {}", test2, parse(test2));
}
