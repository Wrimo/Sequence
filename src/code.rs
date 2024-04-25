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
        match (x.left_prev, x.right_prev) {
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
                    "IfState" => {
                        statement.reset();
                    }

                    "IfState4" => {
                        // according to the grammar, IfState4 is the one with the StatementList
                        statement.statement_type = StatementType::IF;
                        let mut if_state: Statement = statement.clone();
                        if_state.code_block = Some(Vec::new());

                        if let Some(ref mut block) = if_state.code_block {
                            generate_abstract_syntax(m, block, statement, i);
                            generate_abstract_syntax(m, block, statement, j);
                        }
                        program.push(if_state);
                        return;
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
                    // similary there is a bug where b <- a does not work
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
        match (x.left_prev, x.right_prev) {
            (Some(i), Some(j)) => {
                let mut l: Box<Expression> = rc_generate_expression(m, i);
                let mut r: Box<Expression> = rc_generate_expression(m, j);

                match *r {
                    Expression::ADD(ref mut x, ref mut y)
                    | Expression::SUB(ref mut x, ref mut y)
                    | Expression::MUL(ref mut x, ref mut y)
                    | Expression::DIV(ref mut x, ref mut y)
                    | Expression::MOD(ref mut x, ref mut y) => {
                        *x = l;
                        return r;
                    }

                    _ => {}
                }

                match *l {
                    Expression::ADD(ref mut x, ref mut _y)
                    | Expression::SUB(ref mut x, ref mut _y)
                    | Expression::MUL(ref mut x, ref mut _y)
                    | Expression::DIV(ref mut x, ref mut _y)
                    | Expression::MOD(ref mut x, ref mut _y) => {
                        *_y = r;
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
                    TokenType::EQUALOP => {
                        return Box::new(Expression::EQU(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::NOTEQUALOP => {
                        return Box::new(Expression::NEQU(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::GTHANOP => {
                        return Box::new(Expression::GTH(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::GETHANOP => {
                        return Box::new(Expression::GTHE(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::LTHANOP => {
                        return Box::new(Expression::LTH(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::LETHANOP => {
                        return Box::new(Expression::LTHE(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                    }
                    TokenType::PREV => return Box::new(Expression::PREV(String::from(""))),
                    _ => {}
                };
            }
        }
    }
    return Box::new(Expression::NONE);
}

fn execute_program(program: &Vec<Statement>, memory: &mut HashMap<String, Vec<i32>>) {
    for statement in program {
        println!("{:?}", statement);
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let name: String = statement.var_name.clone().unwrap();
                memory.entry(name).and_modify(|ent| ent.push(val)).or_insert(vec![val]);
            }
            StatementType::PRINT => {
                // let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let var_history: &Vec<i32> = memory.get(&*statement.var_name.as_ref().unwrap()).unwrap();
                println!("{}", var_history[var_history.len() - 1]);
            }

            StatementType::IF => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                if x >= 1 {
                    execute_program(&statement.code_block.as_ref().unwrap(), memory);
                }
            }

            _ => {}
        }
    }
}

fn calculate_expression(expr: Box<Expression>, memory: &HashMap<String, Vec<i32>>) -> i32 {
    match *expr {
        Expression::ADD(x, y) => calculate_expression(x, memory) + calculate_expression(y, memory),
        Expression::SUB(x, y) => calculate_expression(x, memory) - calculate_expression(y, memory),
        Expression::MUL(x, y) => calculate_expression(x, memory) * calculate_expression(y, memory),
        Expression::DIV(x, y) => calculate_expression(x, memory) / calculate_expression(y, memory),
        Expression::MOD(x, y) => calculate_expression(x, memory) % calculate_expression(y, memory),
        Expression::EQU(x, y) => (calculate_expression(x, memory) == calculate_expression(y, memory)) as i32,
        Expression::NEQU(x, y) => (calculate_expression(x, memory) != calculate_expression(y, memory)) as i32,
        Expression::LTH(x, y) => (calculate_expression(x, memory) < calculate_expression(y, memory)) as i32,
        Expression::LTHE(x, y) => (calculate_expression(x, memory) <= calculate_expression(y, memory)) as i32,
        Expression::GTH(x, y) => (calculate_expression(x, memory) > calculate_expression(y, memory)) as i32,
        Expression::GTHE(x, y) => (calculate_expression(x, memory) >= calculate_expression(y, memory)) as i32,

        Expression::INTEGER(x) => x,
        Expression::IDENTIFIER(s) => {
            let var_history: &Vec<i32> = memory.get(&s).unwrap();
            var_history[var_history.len() - 1]
        }

        Expression::PREV(s) => {
            let var_history: &Vec<i32> = memory.get(&s).unwrap();
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
                code_block: None,
            };

            generate_abstract_syntax(&m, &mut program, &mut statement, ent.left_prev.unwrap());
            generate_abstract_syntax(&m, &mut program, &mut statement, ent.right_prev.unwrap());
            break;
        }
    }
    let mut memory: HashMap<String, Vec<i32>> = HashMap::new();
    execute_program(&program, &mut memory);
}
