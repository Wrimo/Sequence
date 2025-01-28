mod interpreter;
mod user_options;
mod command_line;

use std::{env, fs, process};
use std::path::PathBuf;

use crate::interpreter::executor::run_program;

fn main() {
    // TODO:
    // [x] better error messages
    // [] tests
    // [] clean up modules (need to do this again)
    // [x] rewrite parser
    // [] add an option print if the a file is running at the top level

    let result = command_line::handle_args(&env::args().collect());
    
    
    let buf = fs::read_to_string(&result.file_name).unwrap_or_else(|_| {
        eprintln!("could not read file: {}", &result.file_name);
        process::exit(1);
    });

    let path = PathBuf::from(&result.file_name);

    run_program(&buf, &path, result.parameters);
}
