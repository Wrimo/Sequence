use std::collections::HashMap;

use crate::code_types::{Expression, Statement, StatementType};
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
                    "Expr" => {
                        statement.expr = Some(generate_expression(m, index));
                        return;
                    }

                    _ => {} // should add error case here
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
                    TokenType::IDENTIFIER(s) => statement.var_name = Some(s.clone()),
                    TokenType::INTEGER(x) => statement.expr = Some(Box::new(Expression::INTEGER(x.clone()))), // is there a way to make this part of generate_expression?
                    // currently it is not called for cases like since it is not a terminal symbol
                    _ => {}
                }
            }
        }
    }
}

fn generate_expression(m: &Vec<Vec<Vec<CYKEntry>>>, index: (usize, usize)) -> Box<Expression> {
    return rc_generate_expression(m, index);
}

fn rc_generate_expression(m: &Vec<Vec<Vec<CYKEntry>>>, index: (usize, usize)) -> Box<Expression> {
    for x in &m[index.0][index.1] {
        match (x.prev, x.prev1) {
            (Some(i), Some(j)) => {
                let mut r: Box<Expression> = rc_generate_expression(m, j);
                let mut l: Box<Expression> = rc_generate_expression(m, i);

                match *r {
                    Expression::ADD(ref _x, ref mut y)
                    | Expression::SUB(ref _x, ref mut y)
                    | Expression::MUL(ref _x, ref mut y)
                    | Expression::DIV(ref _x, ref mut y)
                    | Expression::MOD(ref _x, ref mut y) => {
                        *y = l;
                        return r;
                    }

                    _ => {}
                }
                
                match *l {
                    Expression::ADD(ref mut x, ref _y)
                    | Expression::SUB(ref mut x, ref _y)
                    | Expression::MUL(ref mut x, ref _y)
                    | Expression::DIV(ref mut x, ref _y)
                    | Expression::MOD(ref mut x, ref _y) => {
                        *x = r;
                        return l;
                    }

                    Expression::PREV(ref mut s) => {
                        if let Expression::IDENTIFIER(s1) = *r {
                            *s = s1;
                        }
                        return l;
                    }

                    _ => {}
                }
            }
            _ => {
                match &x.token.token_type {
                    TokenType::INTEGER(x) => return Box::new(Expression::INTEGER(x.clone())),
                    TokenType::IDENTIFIER(s) => return Box::new(Expression::IDENTIFIER(s.clone())),

                    TokenType::ADDOP => return Box::new(Expression::ADD(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                    TokenType::SUBOP => return Box::new(Expression::SUB(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                    TokenType::MULOP => return Box::new(Expression::MUL(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                    TokenType::DIVOP => return Box::new(Expression::DIV(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                    TokenType::MODOP => return Box::new(Expression::MOD(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                    TokenType::PREV => return Box::new(Expression::PREV(String::from(""))),
                    _ => {}
                };
            }
        }
    }
    return Box::new(Expression::NONE);
}

fn execute_program(program: &Vec<Statement>) {
    let mut memory: HashMap<&str, Vec<i32>> = HashMap::new();

    for statement in program {
        // println!("{:?}", statement);
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                memory
                    .entry(&statement.var_name.as_ref().unwrap() as &str)
                    .and_modify(|ent| ent.push(val))
                    .or_insert(vec![val]);
            }
            StatementType::PRINT => {
                let var_history: &Vec<i32> = memory.get(&statement.var_name.as_ref().unwrap() as &str).unwrap();
                println!("{}", var_history[var_history.len() - 1]);
            }

            _ => {}
        }
    }
}

fn calculate_expression(expr: Box<Expression>, memory: &HashMap<&str, Vec<i32>>) -> i32 {
    match *expr {
        Expression::ADD(x, y) => calculate_expression(x, memory) + calculate_expression(y, memory),
        Expression::SUB(x, y) => calculate_expression(x, memory) - calculate_expression(y, memory),
        Expression::MUL(x, y) => calculate_expression(x, memory) * calculate_expression(y, memory),
        Expression::DIV(x, y) => calculate_expression(x, memory) / calculate_expression(y, memory),
        Expression::MOD(x, y) => calculate_expression(x, memory) % calculate_expression(y, memory),

        Expression::INTEGER(x) => x,
        Expression::IDENTIFIER(s) => {
            let var_history: &Vec<i32> = memory.get(&*s).unwrap();
            var_history[var_history.len() - 1]
        }

        Expression::PREV(s) => {
            let var_history: &Vec<i32> = memory.get(&*s).unwrap();
            var_history[var_history.len() - 2]
        }

        Expression::NONE => 0,
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
    println!("Parsing succeeded!");

    let mut program: Vec<Statement> = Vec::new();
    for ent in &m[0][m.len() - 1] {
        if ent.symbol == "S" {
            let mut statement: Statement = Statement {
                statement_type: StatementType::NONE,
                var_name: None,
                expr: None,
            };

            generate_abstract_syntax(&m, &mut program, &mut statement, ent.prev.unwrap());
            generate_abstract_syntax(&m, &mut program, &mut statement, ent.prev1.unwrap());
            break;
        }
    }
    execute_program(&program);
}
