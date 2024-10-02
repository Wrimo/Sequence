use std::{env, fs, process};
use crate::interpreter::runtime_types::{History, HistoryCollection, VariableType};
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
        let mut history: History = History::new(); 
        let value: VariableType;

        if let Ok(x) = args[i].parse::<f64>() { 
            value = VariableType::FLOAT(x);
        }
        else if let Ok(x) = args[i].parse::<i64>() {
            value = VariableType::INTEGER(x);
        } 
        else {
            eprintln!("invalid parameter");
            panic!();
        }

        history.add(value);
        histories.push(history);
    }

    return Some(histories);
}

fn usage(progname: &String) {
    eprintln!("Usage:");
    eprintln!("  {progname} [-d] <source> [parameters]6");
    eprintln!("  -d: debug print");
    process::exit(1);
}