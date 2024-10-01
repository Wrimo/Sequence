mod interpreter;
mod user_options;

use std::{env, fs, process};
use user_options::USER_OPTIONS;
use std::path::PathBuf;

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
    // [x] better error messages
    // [] tests
    // [] clean up modules
    // [x] rewrite parser
    // BUGS:
    // [] a <- 2 * not false fails the parsers

    // current prog calls prog2 calls prog3 which is a copy of sort.sq takes
    // 8 seconds to run. how much can that be reduced by improving call procedure?

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage(&args[0])
    };
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

    let mut path = PathBuf::from(&args[args.len() - 1]);
    PathBuf::pop(&mut path);

    run_program(&buf, &path);
}
