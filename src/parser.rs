use crate::types::{Production, ProductionOption, Token, TokenType};
use std::collections::HashSet;
use std::io::{self, Read};
use std::{fs, vec};

fn symbol_anlysis() -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut input: String = String::from("");
    let _ = io::stdin().read_to_string(&mut input);
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        let chr = chars[i];
        let mut token = Token {
            token_type: TokenType::NONE,
        };

        if chr == ' ' || chr == '\t' {
            i += 1;
            continue;
        }
        if chr == '\n' {
            token.token_type = TokenType::NEWLINE;
        }
        if chr == ';' {
            token.token_type = TokenType::SEMICOLON;
        } else if chr == '<' && input.chars().nth(i + 1)? == '-' {
            token.token_type = TokenType::ASSIGNMENT;
            i += 1;
        } else if chr == '(' {
            token.token_type = TokenType::RPAREN;
        } else if chr == ')' {
            token.token_type = TokenType::LPAREN;
        } else if chr.is_alphabetic() {
            let mut j = i + 1;
            while j < input.len() && chars[j].is_alphanumeric() {
                j += 1;
            }
            // need to add key word check 
            token.token_type = TokenType::IDENTIFIER(input[i..j].to_string());
            i = j - 1;
        } else if chr.is_numeric() {
            let mut j = i + 1;
            while j < input.len() && chars[j].is_numeric() {
                j += 1;
            }
            token.token_type = TokenType::INTEGER(input[i..j].parse().unwrap());
            i = j - 1;
        }
        i += 1;
        tokens.push(token);
    }
    return Some(tokens);
}

fn get_productions() -> Vec<Production> {
    // need to convert to using tokens type instead of string
    let tokens = symbol_anlysis().unwrap();

    for x in tokens {
        println!("{:?}", x);
    }

    let mut productions: Vec<Production> = Vec::new();

    let grammar = fs::read_to_string("../grammar").unwrap();
    for line in grammar.split("\n") {
        let words: Vec<&str> = line.split_whitespace().collect();

        assert!(words.len() >= 3);

        let symbol: &str = words[0];
        let mut options: Vec<ProductionOption> = Vec::new();

        let mut i = 2; // 2 since 0 is the production's symbol and 1 is ->
        while i < words.len() {
            let a: &str = words[i];
            let mut b: &str = "|";
            if i + 1 < words.len() {
                b = words[i + 1];
            }

            let mut opt = ProductionOption {
                production: Some(a.to_string()),
                production1: None,
            };

            if b == "|" {
                // this option has only one symbol
                i += 2;
            } else {
                opt.production1 = Some(b.to_string());
                i += 3;
            }

            options.push(opt);
        }

        let prod: Production = Production {
            symbol: symbol.to_string(),
            produces: options,
        };

        productions.push(prod);
    }
    return productions;
}

#[allow(non_snake_case)]
pub fn parse(input: &str) -> bool {
    let grammar: Vec<Production> = get_productions();
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut M: Vec<Vec<HashSet<String>>> = vec![vec![HashSet::new(); words.len()]; words.len()];

    for i in 0..words.len() {
        for r in 0..grammar.len() {
            if grammar[r].goes_to_terminal(&words[i]) {
                M[i][i].insert(grammar[r].symbol.clone());
            }
        }
    }

    for l in 1..(words.len()) {
        for r in 0..(words.len() - l) {
            for t in 0..(l) {
                let L: HashSet<String> = M[r][r + t].clone();
                let R: HashSet<String> = M[r + t + 1][r + l].clone();

                for b in L.iter() {
                    for c in R.iter() {
                        for prod in grammar.iter() {
                            if prod.goes_concatted(&b, &c) {
                                M[r][r + l].insert(prod.symbol.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    return M[0][words.len() - 1].contains("S");
}
