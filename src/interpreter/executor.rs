use std::collections::HashMap;
use super::interpreter;
use super::parser::lexer::symbol_analysis;
use super::parser::parse::Parser;
use super::parser::statement::Program;
// this manages storing programs that correspond to specific files 
// so they do have to be reevaluated each time the file is run

pub fn run_program(input: &str) {
    let tokens = symbol_analysis(input).unwrap(); // better errors later
    let mut parser = Parser::new(tokens);
    let prog = parser.run(); 
    let mut statement_cache: HashMap<String, Program> = HashMap::new(); 
    // i think there's potential for more speed up by limiting when a prog is cloned currently it
    // 1. write the program when parsing happens
    // 2. clone it into the cache 
    // 3. every time (including first) we execute program 
    // i think there should be a way to pass a reference to the program in the HashMap rather than cloning it 
    // but it was running into issues with multiple conflicting (mut/immut) references 
    // maybe use lazy_static crate to make global cache?
    statement_cache.insert(input.to_string(), prog.clone()); 
    interpreter::execute_program(prog, &mut statement_cache); 
}