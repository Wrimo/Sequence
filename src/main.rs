mod code;
mod code_types;
mod parser;
mod parsing_types;

use code::run_program;
use std::{fs, io::{self, Read}};

fn main() {
    // let mut buf = String::new();
    let buf = fs::read_to_string("prog").unwrap();
    // let _ = io::stdin().read_to_string(&mut buf);
    // expand CYK to allow doing A -> B? Would make writing grammar easier, but might break something else.

    run_program(&buf);
}
