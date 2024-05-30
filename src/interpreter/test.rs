use std::collections::HashMap;

use super::code_types::{Expression, VariableType};
use super::executor::calculate_expression;

#[test]
fn test_simple_add() {
    let x = Box::new(Expression::ADD(
        Box::new(Expression::INTEGER(2)),
        Box::new(Expression::INTEGER(2)),
    ));
    let m: HashMap<String, Vec<VariableType>> = HashMap::new(); 
    let y = calculate_expression(x, &m); 

    assert!(matches!(y, VariableType::INTEGER(4)));
}
