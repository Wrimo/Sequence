mod program;
mod code_types;
mod parser;
mod parsing_types;
mod executor;

use std::{fs, env, process};

use executor::run_program; 

fn usage(progname: &String) {
    eprintln!("Usage:");
    eprintln!("  {progname} <source>");
    process::exit(1);
}

fn main() {
    // TODO: 
    // [x] equal operator
    // [] else block
    // [x] loop of program
    // [x] begin block
    // [x] expect block
    // [] better error messages
    // BUGS: 
    // [] a <- 12 + 12 * 12 fails the parser 

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage(&args[0]);
    }

    let buf = fs::read_to_string(&args[1]).unwrap_or_else(|_| {
        eprintln!("could not read file: {}", args[1]);
        process::exit(1);
    });

    run_program(&buf);
}
