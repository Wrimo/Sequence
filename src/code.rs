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
                let mut r: Box<Expression> = rc_generate_expression(m, i);
                let mut l: Box<Expression> = rc_generate_expression(m, j);

                // if the right returned an operator, we want to it return it as the result for this node
                // with the the left to left

                match *l {
                    // check left first because grammar expands to left (Expr AddOp_Term)
                    Expression::ADD(ref mut x, ref _y)
                    | Expression::SUB(ref mut x, ref _y)
                    | Expression::MUL(ref mut x, ref _y)
                    | Expression::DIV(ref mut x, ref _y) 
                    | Expression::MOD(ref mut x, ref _y) => {
                        *x = r;
                        return l;
                    }
                    _ => {}
                }

                match *r {
                    Expression::ADD(ref _x, ref mut y)
                    | Expression::SUB(ref _x, ref mut y)
                    | Expression::MUL(ref _x, ref mut y)
                    | Expression::DIV(ref _x, ref mut y) 
                    | Expression::MOD(ref _x, ref mut y)=> {
                        *y = l;
                        return r;
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
                    _ => {}
                };
            }
        }
    }
    return Box::new(Expression::NONE);
}

fn execute_program(program: &Vec<Statement>) {
    let mut memory: HashMap<&str, i32> = HashMap::new();

    for statement in program {
        println!("{:?}", statement);
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                memory
                    .entry(statement.var_name.as_ref().unwrap())
                    .and_modify(|x| *x = val)
                    .or_insert(val);
            }
            StatementType::PRINT => {
                println!("{}", memory.entry(statement.var_name.as_ref().unwrap()).or_insert(0));
            }

            _ => {}
        }
    }
}

fn calculate_expression(expr: Box<Expression>, memory: &HashMap<&str, i32>) -> i32 {
    match *expr {
        Expression::ADD(x, y) => calculate_expression(x, memory) + calculate_expression(y, memory),
        Expression::SUB(x, y) => calculate_expression(x, memory) - calculate_expression(y, memory),
        Expression::MUL(x, y) => calculate_expression(x, memory) * calculate_expression(y, memory),
        Expression::DIV(x, y) => calculate_expression(x, memory) / calculate_expression(y, memory),
        Expression::MOD(x, y) => calculate_expression(x, memory) % calculate_expression(y, memory),

        Expression::INTEGER(x) => x,
        Expression::IDENTIFIER(s) => *memory.get(&*s).unwrap(),

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
