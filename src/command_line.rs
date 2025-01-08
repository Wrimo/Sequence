
use std::cell::RefCell;
use std::rc::Rc;
use std::{process};
use crate::interpreter::runtime_types::{History, HistoryCollection, SharedHistory, VariableType};
use crate::interpreter::parser::parsing_types::{Token, TokenType};
use crate::interpreter::parser::lexer::symbol_analysis;
use crate::user_options::USER_OPTIONS;

pub struct ArgResult {
    pub file_name: String,
    pub parameters: Option<HistoryCollection>,
}

pub fn handle_args(args: &Vec<String>) -> ArgResult {
    let mut result: ArgResult = ArgResult {
        file_name: String::new(),
        parameters: None,
    };

    if args.len() < 2 {
        usage(&args[0])
    };

    let mut paramter_index = 0;
    for i in 1..args.len() {
        if args[i].starts_with("-") {
            match args[i].as_str() {
                "-d" => USER_OPTIONS.lock().unwrap().debug = true,
                _ => {},
            }
            continue;
        }

        result.file_name = args[i].clone();
        paramter_index = i + 1;
        break;
    }

    result.parameters = get_parameters(args, paramter_index);

    return result;
}

fn get_parameters(args: &Vec<String>, index: usize) -> Option<HistoryCollection> {
    if index >= args.len() {
        return None; 
    }

    let mut histories: HistoryCollection = HistoryCollection::new();
    for i in index..args.len() {

        let tokens: Option<Vec<Token>>= symbol_analysis(args[i].as_str());
        let history = parse_history(tokens.unwrap());
        
        histories.push(history);
    }

    return Some(histories);
}

// history syntax 
// {1, 2, 3, 4, 10}
fn parse_history(tokens: Vec<Token>) -> SharedHistory { 
    let mut history = History::new();

    assert!(tokens[0].token_type == TokenType::LBRACKET);

    for i in (1..tokens.len()).step_by(2) { 
        match &tokens[i].token_type {
            TokenType::FLOAT(x) => history.add(VariableType::FLOAT(*x)),
            TokenType::INTEGER(x) => history.add(VariableType::INTEGER(*x)),
            TokenType::STRING(x) => history.add(VariableType::STRING(x.clone())),

            _ => panic!("bad token in parameter {:?}", tokens[i].token_type),
        };

        assert!(tokens[i+1].token_type == TokenType::COMMA || tokens[i+1].token_type == TokenType::RBRACKET);
    }
    assert!(tokens[tokens.len()-1].token_type == TokenType::RBRACKET);
    return Rc::new(RefCell::new(history));
}


fn usage(progname: &String) {
    eprintln!("Usage:");
    eprintln!("  {progname} [-d] <source> [parameters]6");
    eprintln!("  -d: debug print");
    process::exit(1);
}