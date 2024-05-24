use crate::code_types::{Expression, Program, Statement, StatementType};
use crate::parsing_types::{CYKEntry, TokenType};

pub fn generate_abstract_syntax(
    start: Box<CYKEntry>,
    body: &mut Vec<Statement>,
    program: &mut Program,
    statement: &mut Statement,
) {
    rc_generate_abstract_syntax(start.left_prev.unwrap(), body, program, statement);
    rc_generate_abstract_syntax(start.right_prev.unwrap(), body, program, statement);
}

fn rc_generate_abstract_syntax(
    production: Box<CYKEntry>,
    code_block: &mut Vec<Statement>,
    program: &mut Program,
    statement: &mut Statement,
) {
    let symbol = production.symbol.clone();
    // println!("{}", symbol);

    if symbol == "<Expr>" { 
        statement.expr = Some(generate_expression(production));
        return;
    }

    match (&production.left_prev, &production.right_prev) {
        // nonterminals
        (Some(i), Some(j)) => {
            // cringe awful solution to hardcode that, but just checking for CodeBlock which trigger this
            // code multiple times. would like to make a better approach, but might require moving away 
            // from a cnf grammar and hence rewriting the whole parser
            if symbol == "<$02<CodeBlock>>" && statement.statement_type.has_code_block() {
                let mut cur_state: Statement = statement.clone();
                cur_state.code_block = Some(Vec::new());    
                if let Some(ref mut block) = cur_state.code_block {
                    rc_generate_abstract_syntax(production.left_prev.unwrap(), block, program, statement);
                    rc_generate_abstract_syntax(production.right_prev.unwrap(), block, program, statement);
                }

                match cur_state.statement_type {
                    StatementType::EXPECT => program.expect = Some(cur_state),
                    StatementType::BEGIN => program.begin = Some(cur_state),
                    _ => code_block.push(cur_state),
                }
                return;
            } 
            rc_generate_abstract_syntax(i.clone(), code_block, program, statement);
            rc_generate_abstract_syntax(j.clone(), code_block, program, statement);
        }
        // terminals
        _ => {
            match &production.token.token_type {
                TokenType::NEWLINE => {
                    // only want to push statements here that do not have corresponding code blocks
                    if statement.statement_type != StatementType::NONE && !statement.statement_type.has_code_block() {
                        code_block.push(statement.clone());
                        statement.reset();
                    }
                }

                TokenType::BEGIN => statement.statement_type = StatementType::BEGIN,
                TokenType::EXPECT => statement.statement_type = StatementType::EXPECT,
                TokenType::IF => statement.statement_type = StatementType::IF,
                TokenType::ASSIGNMENT => statement.statement_type = StatementType::ASSIGN,
                TokenType::PRINT => statement.statement_type = StatementType::PRINT,
                TokenType::REVEAL => statement.statement_type = StatementType::REVEAL,
                TokenType::IDENTIFIER(s) => statement.var_name = Some(s.clone()),

                _ => {}
            }
            return;
        }
    }
}

fn generate_expression(production: Box<CYKEntry>) -> Box<Expression> {
    return rc_generate_expression(production);
}

fn rc_generate_expression(production: Box<CYKEntry>) -> Box<Expression> {
    // println!("{:?}", production.symbol);
    match (&production.left_prev, &production.right_prev) {
        (Some(i), Some(j)) => {
            let mut l: Box<Expression> = rc_generate_expression(i.clone());
            let mut r: Box<Expression> = rc_generate_expression(j.clone());

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
            match &production.token.token_type {
                TokenType::INTEGER(x) => return Box::new(Expression::INTEGER(x.clone())),
                TokenType::IDENTIFIER(s) => return Box::new(Expression::IDENTIFIER(s.clone())),

                TokenType::ADDOP => return Box::new(Expression::ADD(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::SUBOP => return Box::new(Expression::SUB(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::MULOP => return Box::new(Expression::MUL(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::DIVOP => return Box::new(Expression::DIV(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::MODOP => return Box::new(Expression::MOD(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::EQUALOP => return Box::new(Expression::EQU(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::NOTEQUALOP => {
                    return Box::new(Expression::NEQU(Box::new(Expression::NONE), Box::new(Expression::NONE)))
                }
                TokenType::GTHANOP => return Box::new(Expression::GTH(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::GETHANOP => return Box::new(Expression::GTHE(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::LTHANOP => return Box::new(Expression::LTH(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::LETHANOP => return Box::new(Expression::LTHE(Box::new(Expression::NONE), Box::new(Expression::NONE))),
                TokenType::PREV => return Box::new(Expression::PREV(String::from(""))),
                _ => {}
            };
        }
    }
    return Box::new(Expression::NONE);
}
