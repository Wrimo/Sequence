mod code;
mod code_types;
mod parser;
mod parsing_types;

use std::fs;

use code::run_program;

fn main() {
    // TODO: add a precprocessing step that turns \n \n \n to \n
    let buf = fs::read_to_string("prog").unwrap();
    run_program(&buf);
}
