use crate::types::{Production, ProductionOption};
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn get_productions() -> Vec<Production> {
    let stdin = io::stdin();
    let mut productions: Vec<Production> = Vec::new();
    for line in stdin.lock().lines() {
        let line: String = line.unwrap();
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
pub fn parse(grammar: &Vec<Production>, input: &str) -> bool {
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
