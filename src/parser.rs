use crate::parsing_types::{CYKEntry, ConcattedProductions, Production, Token, TokenType};
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, vec};

fn symbol_analysis(input: &str) -> Option<Vec<Token>> {
    let symtbl: HashMap<&str, TokenType> = vec![
        ("<-", TokenType::ASSIGNMENT),
        ("==", TokenType::EQUALOP),
        ("!=", TokenType::NOTEQUALOP),
        (">=", TokenType::GETHANOP),
        ("<=", TokenType::LETHANOP),
        ("\n", TokenType::NEWLINE),
        (";", TokenType::SEMICOLON),
        (")", TokenType::LPAREN),
        ("(", TokenType::RPAREN),
        ("}", TokenType::LBRACKET),
        ("{", TokenType::RBRACKET),
        ("+", TokenType::ADDOP),
        ("-", TokenType::SUBOP),
        ("*", TokenType::MULOP),
        ("%", TokenType::MODOP),
        ("/", TokenType::DIVOP),
        (">", TokenType::GTHANOP),
        ("<", TokenType::LTHANOP),
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
    let mut i = 0;

    while i < chars.len() {
        let chr = chars[i];

        if chr == ' ' || chr == '\t' {
            i += 1;
            continue;
        }

        let mut token = Token {
            token_type: TokenType::NONE,
        };

        // is_sym will handle incrementing `i`.
        if let Some(t) = is_sym(&mut i, max_symlen) {
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
    // due to constraints of chomsky normal form (or my own inexperience)
    // program is expected to be ended with a new line
    if tokens.len() > 1 {
        tokens.push(Token {
            token_type: TokenType::NEWLINE,
        });
    }
    return Some(tokens);
}

fn get_productions() -> Vec<Production> {
    let mut productions: Vec<Production> = Vec::new();

    let grammar = fs::read_to_string("grammar.cnf").unwrap();
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

    if USER_OPTIONS.lock().unwrap().debug {
        for x in &tokens {
            println!("{:?}", x);
        }
    }

    let mut M: Vec<Vec<Vec<CYKEntry>>> = vec![vec![Vec::new(); tokens.len()]; tokens.len()];
    for i in 0..tokens.len() {
        for r in 0..grammar.len() {
            if grammar[r].goes_to_terminal(&tokens[i]) {
                let ent = CYKEntry {
                    symbol: grammar[r].symbol.clone(),
                    left_prev: None,
                    right_prev: None,
                    token: tokens[i].clone(),
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
                                    left_prev: Some(Box::new(b.clone())),
                                    right_prev: Some(Box::new(c.clone())),
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
    if USER_OPTIONS.lock().unwrap().debug {
        for i in 0..M.len() {
            for j in 0..M[i].len() {
                print!("{{");
                for x in &M[i][j] {
                    print!("{} ", x.symbol);
                }
                print!("}}");
            }
            println!();
        }
    }

    for ent in &M[0][tokens.len() - 1] {
        if ent.symbol == "<$S>" {
            return Ok(M);
        }
    }
    return Err(());
}
