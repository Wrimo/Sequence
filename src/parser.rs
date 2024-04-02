use crate::parsing_types::{CYKEntry, ConcattedProductions, Production, Token, TokenType};
use std::collections::HashSet;
use std::str::FromStr;
use std::{fs, vec};

fn symbol_anlysis(input: &str) -> Option<Vec<Token>> {
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
        } else if chr == '+' {
            token.token_type = TokenType::ADDOP;
        } else if chr == '-' {
            token.token_type = TokenType::SUBOP;
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
    tokens.push(Token {
        // adding an arbitary NEWLINE lets the grammar assume every statement is followed by a newline
        token_type: TokenType::NEWLINE,
    });
    return Some(tokens);
}

fn get_productions() -> Vec<Production> {
    let mut productions: Vec<Production> = Vec::new();

    let grammar = fs::read_to_string("grammar").unwrap();
    for line in grammar.split("\n") {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() == 0 {
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
                let token = TokenType::from_str(a).unwrap();

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

#[allow(non_snake_case)]
pub fn parse(input: &str) -> bool {
    let grammar: Vec<Production> = get_productions();
    let tokens: Vec<Token> = match symbol_anlysis(input) {
        Some(x) => x,
        None => return false,
    };

    for x in &tokens {
        println!("{:?}", x);
    }

    let mut M: Vec<Vec<HashSet<CYKEntry>>> = vec![vec![HashSet::new(); tokens.len()]; tokens.len()];

    for i in 0..tokens.len() {
        for r in 0..grammar.len() {
            if grammar[r].goes_to_terminal(&tokens[i]) {
                let ent = CYKEntry {
                    symbol: grammar[r].symbol.clone(),
                    prev: None,
                    prev1: None,
                };
                M[i][i].insert(ent);
            }
        }
    }

    for l in 1..(tokens.len()) {
        for r in 0..(tokens.len() - l) {
            for t in 0..(l) {
                let L: HashSet<CYKEntry> = M[r][r + t].clone();
                let R: HashSet<CYKEntry> = M[r + t + 1][r + l].clone();

                for b in L.iter() {
                    for c in R.iter() {
                        for prod in grammar.iter() {
                            if prod.goes_to_nonterminal(&b.symbol, &c.symbol) {
                                let ent = CYKEntry {
                                    symbol: prod.symbol.clone(),
                                    prev: Some((r, r + t)),
                                    prev1: Some((r + t + 1, r + l)),
                                };

                                M[r][r + l].insert(ent);
                            }
                        }
                    }
                }
            }
        }
    }

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

    for ent in &M[0][tokens.len() - 1] {
        if ent.symbol == "S" {
            let i = ent.prev.unwrap();
            let j = ent.prev1.unwrap();
            println!("S: {:?} {:?}", i, j);
            show_parse_tree(&M, i);
            show_parse_tree(&M, j);
            return true;
        }
    }
    return false;
}

fn show_parse_tree(M: &Vec<Vec<HashSet<CYKEntry>>>, index: (usize, usize)) {
    for x in &M[index.0][index.1] {
        if x.symbol == "S" { // we only care about the S entry at the start point 
            continue;        // other entries only exist since S must repeat some productions due to Chomsky Normal Form constraints
        }
        match (x.prev, x.prev1) {
            (Some(i), Some(j)) => {
                println!("{} @ {:?}: {:?} {:?}", x.symbol, index, i, j);
                show_parse_tree(M, i);
                show_parse_tree(M, j);
            }
            _ => println!("{}", x.symbol), // temrinal
        }
    }
}
