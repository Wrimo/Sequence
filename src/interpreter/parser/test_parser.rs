use std::{collections::HashMap, path::PathBuf};

use crate::interpreter::{parser::expr::{Expression, ExpressionType}, test};

use super::{lexer, parse::{self, Parser}, parsing_types::{self, TokenType}, statement::{Program, Statement, StatementType}};

fn expect_parse(s: &str, body: Vec<StatementType>) {
    let prog = run_parser(s);

    assert_eq!(prog.body.len(), body.len());
    for i in 0..prog.body.len() {
        assert_eq!(prog.body[i].statement_type, body[i]);
    }
}

fn run_parser(s: &str) ->  Program {
    let mut prog_cache: HashMap<String, Box<Program>> = HashMap::new();
    let test_path: PathBuf = PathBuf::new();
    let mut p = Parser::new(lexer::symbol_analysis(&s).unwrap(), &mut prog_cache, &test_path);
    return p.run().clone();
}

#[test]
fn test_parse_assign() {
    let s = "a <- 2";
    expect_parse(s, vec![StatementType::ASSIGN]);
}

#[test]
fn test_many_assigns() {
    let s = "a <- 2\n b<- 4\n c <-1\n d<-1";

    expect_parse(s, vec![StatementType::ASSIGN, StatementType::ASSIGN, StatementType::ASSIGN, StatementType::ASSIGN]);
}

#[test]
fn test_simple_add() {
    let s = "a <- 2 + 2"; 

    let prog = run_parser(s);

    let stat = prog.body[0].clone();
    assert_eq!(stat.statement_type, StatementType::ASSIGN); 
    assert_eq!(stat.var_name, Some(String::from("a")));

    let lhs = Expression::new(ExpressionType::INTEGER(2), None, None);
    let rhs = Expression::new(ExpressionType::INTEGER(2), None, None);
    let expr = Expression::new(ExpressionType::ADD, Some(lhs), Some(rhs));
    assert_eq!(*stat.expr.unwrap(), *expr);
}