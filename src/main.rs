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
    // [] clean up modules
    // [x] rewrite parser
    // BUGS:
    // [x] a <- 2 * not false fails the parsers
    // want to be able to specifiy parameters on command line 
    // so sequence sort.sq [5, 2, 3, 1, 2] 
    // and sequence next_greater.sq [5, 2, 4, 1] 2
    // would both be runnable seperately

    let result = command_line::handle_args(&env::args().collect());
    
    
    let buf = fs::read_to_string(&result.file_name).unwrap_or_else(|_| {
        eprintln!("could not read file: {}", &result.file_name);
        process::exit(1);
    });

    let path = PathBuf::from(&result.file_name);

    run_program(&buf, &path, &result.parameters);
}
