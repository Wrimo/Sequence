mod code_types;
mod executor;
mod parser;
mod parsing_types;
mod program;
mod user_options;

use std::{env, fs, process};
use user_options::USER_OPTIONS;

use executor::run_program;

fn usage(progname: &String) {
    eprintln!("Usage:");
    eprintln!("  {progname} [-d] <source>");
    eprintln!("  -d: debug print");
    process::exit(1);
}

fn main() {
    // TODO:
    // [x] equal operator
    // [x] else block
    // [x] elif
    // [x] true / false
    // [x] loop of program
    // [x] begin block
    // [x] expect block
    // [x] floats
    // [x] bools (needed to get comparisons working)
    // [] logical operators (requires setting up unary operators)
    // [] factorial, exponential operators 
    // [] better error messages
    // [] tests
    // [] clean up modules
    // BUGS:
    // [x] a <- 12 + 12 * 12 fails the parser
    // [] 2 * 1 + 1 evaluates as 3, but 1 + 1 * 2 evaluates as 2
    // [] parantheses in expression do not work 

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
