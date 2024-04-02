mod parsing_types;
mod parser;
use parser::parse;
use std::io::{self, Read}; 


fn main() {
    let mut buf = String::new();
    let _ = io::stdin().read_to_string(&mut buf);
    // expand CYK to allow doing A -> B? Would make writing grammar easier, but might break something else. 

    println!("{}", buf);
    println!("{}", parse(&buf));
}
