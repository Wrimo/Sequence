mod types;
mod parser;
use parser::parse;


fn main() {
    let test: &str = "print(a)\nprint(b)";
    // expand CYK to allow doing A -> B? Would make writing grammar easier, but might break something else. 
    println!("{} -> {}", test, parse(test));
}
