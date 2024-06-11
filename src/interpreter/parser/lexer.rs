use crate::user_options::USER_OPTIONS;

use super::parsing_types::{Token, TokenType};
use std::collections::HashMap;
use std::str::FromStr;
use std::vec;

// given a string, converts it to a vector of corresponding tokens
pub fn symbol_analysis(input: &str) -> Option<Vec<Token>> {
    let symtbl: HashMap<&str, TokenType> = vec![
        ("<-", TokenType::ASSIGNMENT),
        ("==", TokenType::EQUALOP),
        ("!=", TokenType::NOTEQUALOP),
        (">=", TokenType::GETHANOP),
        ("<=", TokenType::LETHANOP),
        (";", TokenType::SEMICOLON),
        ("(", TokenType::LPAREN),
        (")", TokenType::RPAREN),
        ("{", TokenType::LBRACKET),
        ("}", TokenType::RBRACKET),
        ("+", TokenType::ADDOP),
        ("-", TokenType::SUBOP),
        ("*", TokenType::MULOP),
        ("%", TokenType::MODOP),
        ("/", TokenType::DIVOP),
        (">", TokenType::GTHANOP),
        ("<", TokenType::LTHANOP),
        ("|", TokenType::VERTICALBAR),
        ("!", TokenType::FACTORIAL),
        ("^", TokenType::EXPONENT),
        (",", TokenType::COMMA),
        ("::", TokenType::ACCESSOR),
        ("=:", TokenType::COPY),
        ("#", TokenType::LEN),
        ("--", TokenType::COMMENT),
    ]
    .into_iter()
    .collect();

    // The maximum length of a key in the symtbl.
    let max_symlen = symtbl.keys().map(|k| k.len()).max().unwrap_or(0);

    // Check if a string slice from the input is a
    // symbol in the symtbl.
    let is_sym = |i: &mut usize, max_symlen: usize| {
        for j in (0..max_symlen).rev() {
            if *i + j < input.len() {
                if let Some(t) = symtbl.get(&input[*i..=*i + j]) {
                    *i += j;
                    return Some(t);
                }
            }
        }
        // Not a symbol
        None
    };

    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut line_number = 0;
    let mut i = 0;

    while i < chars.len() {
        println!("{}", i);
        let chr = chars[i];

        if chr == ' ' || chr == '\t' {
            i += 1;
            continue;
        }

        let mut token = Token {
            token_type: TokenType::NONE,
            line: line_number,
        };

        if chr == '\n' {
            line_number += 1;
            i += 1;
            token.token_type = TokenType::NEWLINE;

            if tokens[tokens.len() - 1].token_type != TokenType::NEWLINE {
                tokens.push(token);
            }
            continue;
        }
        // is_sym will handle incrementing `i`.
        if let Some(t) = is_sym(&mut i, max_symlen) {
            if *t == TokenType::COMMENT {
                loop { // ignore rest of current line
                    i += 1;
                    if i >= chars.len() || chars[i] == '\n' {
                        break;
                    }
                }
                continue;
            }
            token.token_type = t.clone();
        } else if chr.is_alphabetic() || chr == '_' {
            let mut j = i;
            while j < input.len() - 1 && (chars[j + 1].is_alphanumeric() || chars[j + 1] == '_') {
                j += 1;
                if let Ok(x) = TokenType::from_str(&input[i..j + 1].to_uppercase()) {
                    token.token_type = x;
                }
            }

            if token.token_type == TokenType::NONE {
                token.token_type = TokenType::IDENTIFIER(input[i..j + 1].to_string());
            }

            i = j;
        } else if chr.is_numeric() {
            let digit_scan = |i: usize, chars: &Vec<char>| {
                let mut j = i + 1;
                while j < chars.len() && chars[j].is_numeric() {
                    j += 1;
                }
                return j;
            };

            let mut j = digit_scan(i, &chars);
            if j < chars.len() && chars[j] == '.' {
                j = digit_scan(j, &chars);
                token.token_type = TokenType::FLOAT(input[i..j].parse().unwrap());
            } else {
                token.token_type = TokenType::INTEGER(input[i..j].parse().unwrap());
            }

            i = j - 1;
        }
        i += 1;
        tokens.push(token);
    }
    tokens.push(Token {
        token_type: TokenType::NEWLINE,
        line: line_number + 1,
    });

    if USER_OPTIONS.lock().unwrap().debug {
        for i in 0..tokens.len() {
            println!("{:?}", tokens[i]);
        }
    }
    return Some(tokens);
}
