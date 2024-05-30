use super::parsing::parse;

#[test]
fn parse_constant_integer_assign() { 
    let s = "a <- 1";
    assert!(!matches!(parse(s), Err(())))
}

#[test]
fn parse_constant_float_assign() { 
    let s = "a <- 1.0"; 
    assert!(!matches!(parse(s), Err(())))
}

#[test]
fn parse_bad_integer_assign() { 
    let s = "a = 1";
    assert!(matches!(parse(s), Err(())))
}