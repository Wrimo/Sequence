use crate::parsing_types::{CYKEntry, ConcattedProductions, Production, Token, TokenType};
use std::str::FromStr;
use std::{fs, vec};

fn symbol_analysis(input: &str) -> Option<Vec<Token>> {
    let mut tokens: Vec<Token> = Vec::new();

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
        // would be nice to replace all the operators/keywords with a lookup table
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
        } else if chr == '}' {
            token.token_type = TokenType::LBRACKET;
        } else if chr == '{' {
            token.token_type = TokenType::RBRACKET;
        } else if chr == '+' {
            token.token_type = TokenType::ADDOP;
        } else if chr == '-' {
            token.token_type = TokenType::SUBOP;
        } else if chr == '*' {
            token.token_type = TokenType::MULOP;
        } else if chr == '%' {
            token.token_type = TokenType::MODOP;
        } else if chr == '/' {
            token.token_type = TokenType::DIVOP;
        } else if chr.is_alphabetic() {
            let mut j = i;
            while j < input.len() - 1 && chars[j + 1].is_alphanumeric() {
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
    // prevents the language from requiring an empty new line at the end  
    tokens.push(Token {
        token_type: TokenType::NEWLINE,
    });
    return Some(tokens);
}

fn get_productions() -> Vec<Production> {
    let mut productions: Vec<Production> = Vec::new();

    let grammar = fs::read_to_string("grammar").unwrap();
    for line in grammar.split("\n") {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() == 0 || words[0].chars().nth(0) == Some('#') {
            // empty line
            continue;
        }

        let mut nonterminal: Vec<ConcattedProductions> = Vec::new();
        let mut temrinal: Vec<TokenType> = Vec::new();

        let symbol: &str = words[0];

        let mut i = 2; // 2 since 0 is the production's symbol and 1 is ->
        while i < words.len() {
            let a: &str = words[i];
            let mut b: &str = "|";
            if i + 1 < words.len() {
                b = words[i + 1];
            }

            if b == "|" {
                // non terminal case
                let token = match TokenType::from_str(a) {
                    Ok(x) => x,
                    _ => {
                        println!("undefined symbol {}", a);
                        return productions;
                    } // need to change get_productions to support error. guess i need a more general error pass
                };

                temrinal.push(token);
                i += 2;
            } else {
                let concatted = ConcattedProductions {
                    production: a.to_string(),
                    production1: b.to_string(),
                };

                nonterminal.push(concatted);
                i += 3;
            }
        }

        let prod: Production = Production {
            symbol: symbol.to_string(),
            nonterminals: nonterminal,
            terminals: temrinal,
        };

        productions.push(prod);
    }
    return productions;
}

type ParseError = ();
#[allow(non_snake_case)]
pub fn parse(input: &str) -> Result<Vec<Vec<Vec<CYKEntry>>>, ParseError> {
    let grammar: Vec<Production> = get_productions();
    let tokens: Vec<Token> = match symbol_analysis(input) {
        Some(x) => x,
        None => return Err(()),
    };

    // for x in &tokens {
    //     println!("{:?}", x);
    // }

    let mut M: Vec<Vec<Vec<CYKEntry>>> = vec![vec![Vec::new(); tokens.len()]; tokens.len()];

    for i in 0..tokens.len() {
        for r in 0..grammar.len() {
            if grammar[r].goes_to_terminal(&tokens[i]) {
                let ent = CYKEntry {
                    symbol: grammar[r].symbol.clone(),
                    prev: None,
                    prev1: None,
                    token: tokens[i].clone(), // is there a potential error since tokens gets associated with productions that could go to then, not ones of do - what if two terminals from one terminal?
                };
                M[i][i].push(ent);
            }
        }
    }

    for l in 1..(tokens.len()) {
        for r in 0..(tokens.len() - l) {
            for t in 0..(l) {
                let L: Vec<CYKEntry> = M[r][r + t].clone();
                let R: Vec<CYKEntry> = M[r + t + 1][r + l].clone();

                for b in L.iter() {
                    for c in R.iter() {
                        for prod in grammar.iter() {
                            if prod.goes_to_nonterminal(&b.symbol, &c.symbol) {
                                let ent = CYKEntry {
                                    symbol: prod.symbol.clone(),
                                    prev: Some((r, r + t)),
                                    prev1: Some((r + t + 1, r + l)),
                                    token: Token {
                                        token_type: TokenType::NONE,
                                    },
                                };

                                M[r][r + l].push(ent);
                            }
                        }
                    }
                }
            }
        }
    }
    // for i in 0..M.len() {
    //     for j in 0..M[i].len() {
    //         print!("{{");
    //         for x in &M[i][j] {
    //             print!("{} ", x.symbol);
    //         }
    //         print!("}}");
    //     }
    //     println!();
    // }
    for ent in &M[0][tokens.len() - 1] {
        if ent.symbol == "S" {
            return Ok(M);
        }
    }
    return Err(());
}
