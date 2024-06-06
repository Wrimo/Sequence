use super::{
    lexer::symbol_analysis, parse::Parser, parsing_types::{Token, TokenType}
};

#[test]
fn parse_assign() {
    let tokens = symbol_analysis("reveal a").unwrap(); 


    for i in 0..tokens.len() { 
        println!("{:?}", tokens[i]);
    }

    let mut parse = Parser::new(tokens);

    parse.run();
}

// #[test]
// fn parse_constant_integer_assign() {
//     let s = "a <- 1";
//     assert!(!matches!(parse(s), Err(())))
// }

// #[test]
// fn parse_constant_float_assign() {
//     let s = "a <- 1.0";
//     assert!(!matches!(parse(s), Err(())))
// }

// #[test]
// fn parse_bad_integer_assign() {
//     let s = "a = 1";
//     assert!(matches!(parse(s), Err(())))
// }
