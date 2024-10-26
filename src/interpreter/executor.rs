use std::collections::HashMap;
use std::path::PathBuf;
use super::interpreter;
use super::parser;
use super::parser::parse;
use super::parser::statement::Program;
use super::runtime_types::HistoryCollection;
// this manages storing programs that correspond to specific files 
// so they do have to be reevaluated each time the file is run

pub fn run_program(input: &str, directory: &PathBuf, parameters: Option<HistoryCollection>) {
    let tokens = parser::lexer::symbol_analysis(input).unwrap(); // better errors later
    let mut prog_cache: HashMap<String, Box<Program>> = HashMap::new(); 

    let mut parser = parse::Parser::new(tokens, &mut prog_cache, directory);
    let prog = parser.run(); 

    interpreter::execute_program(prog, None, parameters); 
}