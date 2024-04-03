use std::collections::HashMap;

use crate::code_types::{Statement, StatementType};
use crate::parser::parse;
use crate::parsing_types::{CYKEntry, TokenType};

fn generate_abstract_syntax(
    m: &Vec<Vec<Vec<CYKEntry>>>,
    program: &mut Vec<Statement>,
    statement: &mut Statement,
    index: (usize, usize),
) {
    for x in &m[index.0][index.1] {
        if x.symbol == "S" {
            // we only care about the S entry at the start point
            continue; // other entries only exist since S must repeat some productions due to Chomsky Normal Form constraints
        }
        match (x.prev, x.prev1) {
            (Some(i), Some(j)) => {
                match x.symbol.as_str() {
                    // handle nonterminals
                    "AssignState" => {
                        statement.reset();
                        statement.statement_type = StatementType::ASSIGN;
                    }
                    "PrintState" => {
                        statement.reset();
                        statement.statement_type = StatementType::PRINT;
                    }
                    _ => {}
                }
                generate_abstract_syntax(m, program, statement, i);
                generate_abstract_syntax(m, program, statement, j);
            }
            _ => {
                // handle terminals, equivalent to token

                match &x.token.token_type {
                    TokenType::NEWLINE => {
                        if statement.statement_type != StatementType::NONE {
                            // needed because nonterminals get associated with tokens they could go to even if they don't
                            program.push(statement.clone());
                            statement.reset();
                        }
                    }
                    TokenType::INTEGER(x) => statement.val = Some(x.clone()),
                    TokenType::IDENTIFIER(s) => statement.var_name = Some(s.clone()),

                    _ => {}
                }
            }
        }
    }
}

fn execute_program(program: &Vec<Statement>) {
    let mut memory: HashMap<&str, i32> = HashMap::new();

    for statement in program {
        match statement.statement_type {
            StatementType::ASSIGN => {
                memory.entry(statement.var_name.as_ref().unwrap()).and_modify(|x| *x = statement.val.unwrap()).or_insert(statement.val.unwrap());
            }, 
            StatementType::PRINT => 
            {
                println!("{}", memory.entry(statement.var_name.as_ref().unwrap()).or_insert(0));
            }

            _ => {}
        }
    }
}

pub fn run_program(input: &str) {
    let m = match parse(input) {
        Ok(x) => x,
        Err(()) => {
            println!("Parsing failed!");
            return;
        }
    };

    let mut program: Vec<Statement> = Vec::new();
    for ent in &m[0][m.len() - 1] {
        if ent.symbol == "S" {
            let mut statement: Statement = Statement {
                statement_type: StatementType::NONE,
                var_name: None,
                val: None,
            };
            generate_abstract_syntax(&m, &mut program, &mut statement, ent.prev.unwrap());
            generate_abstract_syntax(&m, &mut program, &mut statement, ent.prev1.unwrap());
            break;
        }
    }

    execute_program(&program);
}
