mod user_options;
mod interpreter;

use std::{env, fs, process};
use user_options::USER_OPTIONS;

use crate::interpreter::executor::run_program; 

fn usage(progname: &String) {
    eprintln!("Usage:");
    eprintln!("  {progname} [-d] <source>");
    eprintln!("  -d: debug print");
    process::exit(1);
}

fn main() {
    // TODO:
    // [x] logical operators (requires setting up unary operators)
    // [x] factorial, exponential operators 
    // [] execute code from other files
    // [] better error messages
    // [] tests
    // [] clean up modules
    // [] rewrite parser
    // BUGS:
    // [] a <- 2 * not false fails the parsers 

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {usage(&args[0])};
    for i in 1..args.len() - 1 {
        match args[i].as_str() {
            "-d" => USER_OPTIONS.lock().unwrap().debug = true,
            _ => {}
        }
    }

    let buf = fs::read_to_string(&args[args.len() - 1]).unwrap_or_else(|_| {
        eprintln!("could not read file: {}", args[args.len() - 1]);
        process::exit(1);
    });

    run_program(&buf);
}
