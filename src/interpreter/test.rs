use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::interpreter::{interpreter::calculate_expression, parser::expr::{Expression, ExpressionType}, runtime_types::{HistoryCollection, Memory, VariableType}};

use super::runtime_types::{History, SharedHistory};


#[test]
fn test_evaluate_simple_add() {
    let x = Expression::new(ExpressionType::ADD, 
        Some(Expression::new(ExpressionType::INTEGER(2), None, None)),
        Some(Expression::new(ExpressionType::INTEGER(2), None, None)),
    );
    let mut m: Memory = Memory::new();
    let y = calculate_expression(x, &mut m); 

    assert!(matches!(y, VariableType::INTEGER(4)));
}

#[test]
fn test_evaluate_simple_mul() {
    let x = Expression::new(ExpressionType::MUL, 
        Some(Expression::new(ExpressionType::INTEGER(2), None, None)),
        Some(Expression::new(ExpressionType::INTEGER(2), None, None)),
    );
    let mut m: Memory = Memory::new();
    let y = calculate_expression(x, &mut m); 

    assert!(matches!(y, VariableType::INTEGER(4)));
}

#[test]
fn test_evaluate_abs_given_negative() {
    let x = Expression::new(ExpressionType::ABS, 
        Some(Expression::new(ExpressionType::INTEGER(-1), None, None)),
        None,
    );
    let mut m: Memory = Memory::new();
    let y = calculate_expression(x, &mut m); 

    assert!(matches!(y, VariableType::INTEGER(1)));
}

fn make_history(values: Vec<VariableType>) -> SharedHistory {
    let mut hist: History = History::new();
    for x in values { 
        hist.add(x);
    }
    
    Rc::new(RefCell::new(hist))
}

// todo -- here to thend is new tests that fail, working to get them to pass
#[test]
fn test_evaluate_prev() { 
    let name = String::from("h");
    let mut m: Memory = Memory::new();
    
    let shared: SharedHistory = make_history(vec![
            VariableType::INTEGER(1), 
            VariableType::INTEGER(2),
        ]
    );
    m.insert_history(name.clone(), shared);

    let expr = Expression::new(ExpressionType::PREV,
        Some(Expression::new(ExpressionType::IDENTIFIER(name.clone()), None, None)),
        None,   
    );

    let val = calculate_expression(expr, &mut m);
    assert!(matches!(val, VariableType::INTEGER(1)));
}

#[test] 
fn test_evaluate_all() {
    let a = String::from("a");
    let mut m: Memory = Memory::new();

    let shared: SharedHistory = make_history(vec![
            VariableType::INTEGER(1), 
            VariableType::INTEGER(2),
            VariableType::INTEGER(3),
        ]
    );

    m.insert_history(a.clone(), shared);

    let expr = Expression::new(ExpressionType::ALL, 
            Some(Expression::new(ExpressionType::IDENTIFIER(a.clone()), None, None)), 
            None);
    
    let val = calculate_expression(expr, &mut m);

    assert!(matches!(val, VariableType::History(shared)));   
}

