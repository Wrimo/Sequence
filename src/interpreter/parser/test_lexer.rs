#[cfg(test)]
use super::{lexer, parsing_types};
#[cfg(test)]
use crate::interpreter::parser::parsing_types::TokenType;
#[cfg(test)]
use crate::interpreter::runtime_types::History;

#[cfg(test)]
fn expect_tokens(tokens: Vec<parsing_types::Token>, expected: Vec<parsing_types::TokenType>) {
    let len = tokens.len();
    assert_eq!(len, expected.len());
    for i in 0..len {
        assert_eq!(tokens[i].token_type, expected[i]);
    }
}

#[test]
fn test_invalid_character() {
    let s = "`";
    let result: Option<Vec<parsing_types::Token>> = lexer::symbol_analysis(s);
    let tokens = result.unwrap();

    expect_tokens(tokens, vec![TokenType::NONE]);
}

#[test]
fn test_identifier() {
    let s = "name a value";
    let tokens = lexer::symbol_analysis(s).unwrap();

    expect_tokens(
        tokens,
        vec![
            TokenType::IDENTIFIER(String::from("name")),
            TokenType::IDENTIFIER(String::from("a")),
            TokenType::IDENTIFIER(String::from("value")),
        ],
    );
}

#[test]
fn test_integer() {
    let s = "1 12 0 1250 875122";
    let tokens = lexer::symbol_analysis(s).unwrap();

    expect_tokens(
        tokens,
        vec![
            TokenType::INTEGER(1),
            TokenType::INTEGER(12),
            TokenType::INTEGER(0),
            TokenType::INTEGER(1250),
            TokenType::INTEGER(875122),
        ],
    );
}

#[test]
fn test_float() {
    let s = "0.0 1.0 2.5 100.005 9999.9999";
    let tokens = lexer::symbol_analysis(s).unwrap();

    expect_tokens(
        tokens,
        vec![
            TokenType::FLOAT(0.0),
            TokenType::FLOAT(1.0),
            TokenType::FLOAT(2.5),
            TokenType::FLOAT(100.005),
            TokenType::FLOAT(9999.9999),
        ],
    );
}

#[test]
fn test_operators() {
    let s = "+ - / * % < > ! ^";
    let tokens = lexer::symbol_analysis(s).unwrap();

    expect_tokens(
        tokens,
        vec![
            TokenType::ADDOP,
            TokenType::SUBOP,
            TokenType::DIVOP,
            TokenType::MULOP,
            TokenType::MODOP,
            TokenType::LTHANOP,
            TokenType::GTHANOP,
            TokenType::FACTORIAL,
            TokenType::EXPONENT,
        ],
    );
}

#[test]
fn test_keywords() {
    let s = "begin expect with and or not";
    let tokens = lexer::symbol_analysis(s).unwrap();

    expect_tokens(
        tokens,
        vec![
            TokenType::BEGIN,
            TokenType::EXPECT,
            TokenType::WITH,
            TokenType::AND,
            TokenType::OR,
            TokenType::NOT,
        ],
    );
}

// generate more tests to ensure the lexer works correctly
#[test]
fn test_string() {
    let s = "\"hello\"";
    let tokens = lexer::symbol_analysis(s).unwrap();
    expect_tokens(tokens, vec![TokenType::STRING(String::from("hello"))]);
}

#[test]
fn test_whitespace() {
    let s = "\n";
    let tokens = lexer::symbol_analysis(s).unwrap();
    expect_tokens(tokens, vec![TokenType::NEWLINE]);
}

#[test]
fn test_comment() {
    let s = "-- This is a comment";
    let tokens = lexer::symbol_analysis(s).unwrap();
    expect_tokens(tokens, vec![]); // comments should be ignored by lexer
}
