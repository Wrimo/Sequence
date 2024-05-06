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
    // TODO: add a precprocessing step that turns \n \n \n to \n
    // what I need for Collatz:
    // [x] equal operator
    // [] else block
    // [] loop of program
    // [] begin block
    // [] expect block

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
