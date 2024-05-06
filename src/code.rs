use std::collections::HashMap;
// use std::env::var;

use crate::code_types::{Expression, Program, Statement, StatementType};
use crate::parser::parse;
use crate::parsing_types::{CYKBacktrack, CYKEntry, TokenType};

fn generate_abstract_syntax(
    m: &Vec<Vec<Vec<CYKEntry>>>,
    code_block: &mut Vec<Statement>,
    program: &mut Program,
    statement: &mut Statement,
    backtrack: CYKBacktrack,
) {
    for x in &m[backtrack.index.0][backtrack.index.1] {
        // probably better to make backtrack some sort of pointer, but I am leaving this for now
        if x.symbol == backtrack.symbol {
            println!("{}", x.symbol);
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
                    statement.statement_type = StatementType::IF;
                }

                "BeginState" => {
                    statement.reset();
                    statement.statement_type = StatementType::BEGIN;
                }

                "ExpectState" => {
                    statement.reset();
                    statement.statement_type = StatementType::EXPECT;
                }

                "RevealState" => {
                    statement.reset();
                    statement.statement_type = StatementType::REVEAL;
                }

                "CodeBlock2" => {
                    // according to the grammar, IfState4 is the one with the StatementList
                    let mut cur_state: Statement = statement.clone();
                    cur_state.code_block = Some(Vec::new());

                    if let Some(ref mut block) = cur_state.code_block {
                        generate_abstract_syntax(m, block, program, statement, x.left_prev.as_ref().unwrap().clone());
                        generate_abstract_syntax(m, block, program, statement, x.right_prev.as_ref().unwrap().clone());
                    }

                    match cur_state.statement_type {
                        StatementType::EXPECT => program.expect = Some(cur_state),
                        StatementType::BEGIN => program.begin = Some(cur_state),
                        _ => code_block.push(cur_state),
                    }
                    return;
                }
                "Expr" => {
                    statement.expr = Some(generate_expression(m, backtrack));
                    return;
                }

                _ => {} // should add error case here
            }
            match (&x.left_prev, &x.right_prev) {
                (Some(i), Some(j)) => {
                    generate_abstract_syntax(m, code_block, program, statement, i.clone());
                    generate_abstract_syntax(m, code_block, program, statement, j.clone());
                }
                _ => {
                    // handle terminals, equivalent to token

                    match &x.token.token_type {
                        TokenType::NEWLINE => {
                            // only want to push statements here that do not have corresponding code blocks
                            match statement.statement_type {
                                StatementType::ASSIGN | StatementType::PRINT | StatementType::REVEAL => {
                                    code_block.push(statement.clone());
                                    statement.reset();
                                }
                                _ => {}
                            }
                        }
                        TokenType::IDENTIFIER(s) => statement.var_name = Some(s.clone()),
                        _ => {}
                    }
                    return;
                }
            }
            return;
        }
    }
}

fn generate_expression(m: &Vec<Vec<Vec<CYKEntry>>>, backtrack: CYKBacktrack) -> Box<Expression> {
    return rc_generate_expression(m, backtrack);
}

fn rc_generate_expression(m: &Vec<Vec<Vec<CYKEntry>>>, backtrack: CYKBacktrack) -> Box<Expression> {
    for x in &m[backtrack.index.0][backtrack.index.1] {
        if x.symbol == backtrack.symbol {
            match (&x.left_prev, &x.right_prev) {
                (Some(i), Some(j)) => {
                    let mut l: Box<Expression> = rc_generate_expression(m, i.clone());
                    let mut r: Box<Expression> = rc_generate_expression(m, j.clone());

                    match *r {
                        Expression::ADD(ref mut x, ref _y)
                        | Expression::SUB(ref mut x, ref _y)
                        | Expression::MUL(ref mut x, ref _y)
                        | Expression::DIV(ref mut x, ref _y)
                        | Expression::MOD(ref mut x, ref _y)
                        | Expression::EQU(ref mut x, ref _y)
                        | Expression::NEQU(ref mut x, ref _y)
                        | Expression::GTH(ref mut x, ref _y)
                        | Expression::GTHE(ref mut x, ref _y)
                        | Expression::LTH(ref mut x, ref _y)
                        | Expression::LTHE(ref mut x, ref _y) => {
                            *x = l;
                            return r;
                        }

                        _ => {}
                    }

                    match *l {
                        Expression::ADD(ref _x, ref mut y)
                        | Expression::SUB(ref _x, ref mut y)
                        | Expression::MUL(ref _x, ref mut y)
                        | Expression::DIV(ref _x, ref mut y)
                        | Expression::MOD(ref _x, ref mut y)
                        | Expression::EQU(ref _x, ref mut y)
                        | Expression::NEQU(ref _x, ref mut y)
                        | Expression::GTH(ref _x, ref mut y)
                        | Expression::GTHE(ref _x, ref mut y)
                        | Expression::LTH(ref _x, ref mut y)
                        | Expression::LTHE(ref _x, ref mut y) => {
                            *y = r;
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

                        TokenType::ADDOP => {
                            return Box::new(Expression::ADD(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                        }
                        TokenType::SUBOP => {
                            return Box::new(Expression::SUB(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                        }
                        TokenType::MULOP => {
                            return Box::new(Expression::MUL(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                        }
                        TokenType::DIVOP => {
                            return Box::new(Expression::DIV(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                        }
                        TokenType::MODOP => {
                            return Box::new(Expression::MOD(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                        }
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
    }
    return Box::new(Expression::NONE);
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

fn execute_program(program: &Vec<Statement>, memory: &mut HashMap<String, Vec<i32>>) {
    for statement in program {
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let name: String = statement.var_name.clone().unwrap();
                memory.entry(name).and_modify(|ent| ent.push(val)).or_insert(vec![val]);
            }
            StatementType::PRINT => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                println!("{}", x);
            }

            StatementType::IF => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                if x >= 1 {
                    execute_program(&statement.code_block.as_ref().unwrap(), memory);
                }
            }

            StatementType::REVEAL => {
                let var_history: &Vec<i32> = memory.get(&statement.var_name.clone().unwrap()).unwrap();
                for i in 0..var_history.len() {
                    println!("{}", var_history[i]);
                }
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
    println!("Parsing succeeded!");

    let mut program: Program = Program {
        begin: None,
        expect: None,
        body: Vec::new(),
    };
    let mut body: Vec<Statement> = Vec::new();
    for ent in &m[0][m.len() - 1] {
        if ent.symbol == "S" {
            let mut statement: Statement = Statement {
                statement_type: StatementType::NONE,
                var_name: None,
                expr: None,
                code_block: None,
            };

            generate_abstract_syntax(
                &m,
                &mut body,
                &mut program,
                &mut statement,
                ent.left_prev.as_ref().unwrap().clone(),
            );
            generate_abstract_syntax(
                &m,
                &mut body,
                &mut program,
                &mut statement,
                ent.right_prev.as_ref().unwrap().clone(),
            );
            program.body = body;
            break;
        }
    }
    let mut memory: HashMap<String, Vec<i32>> = HashMap::new();
    for i in &program.body {
        println!("{:?}", i);
    }
    println!("End body\n\n");

    if matches!(program.expect.as_ref(), None) { 
        println!("WARNING: Running with no expect block, program will not terminate!");
    }
    if let Some(begin) = program.begin {
        println!("begin is {:?}", begin);
        execute_program(&begin.code_block.unwrap(), &mut memory)
    }
    loop {
        execute_program(&program.body, &mut memory);
        // expect block logic
        if let Some(ref expect) = program.expect {
            if calculate_expression(expect.expr.clone().unwrap(), &memory) == 1 {
                execute_program(&expect.code_block.as_ref().unwrap(), &mut memory);
                break;
            }
        }
    }
}
