#[derive(Debug)]

pub enum TokenType {
    NONE,
    IDENTIFIER(String),
    ASSIGNMENT,
    INTEGER(i32),
    STRING(String),
    FLOAT(f32),
    ADDOP,
    SUBOP,
    RPAREN,
    LPAREN,
    SEMICOLON,
    NEWLINE,
    PRINT,
}
#[derive(Debug)]

pub struct Token {
    pub token_type: TokenType,
}

pub struct Production {
    pub symbol: String,
    pub terminals: Vec<Token>,
    pub nonterminals: Vec<ProductionOption>,
}

pub struct ProductionOption {
    pub production: String,
    pub production1: String,
}

impl Production {
    pub fn goes_to_terminal(&self, token: &Token) -> bool {
        for prod_opt in &self.terminals {
            if prod_opt == token {
                return true;
            }
        }
        return false;
    }

    pub fn goes_concatted(&self, sym: &str, sym1: &str) -> bool {
        for prod_opt in &self.produces {
            if prod_opt.production == Some(sym.to_string())
                && prod_opt.production1 == Some(sym1.to_string())
            {
                return true;
            }
        }
        return false;
    }
}
