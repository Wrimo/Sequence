use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TokenType {
    NONE,
    IDENTIFIER(String),
    ASSIGNMENT,
    INTEGER(i32),
    STRING(String),
    FLOAT(f32),
    ADDOP,
    SUBOP,
    MULOP,
    MODOP,
    DIVOP,
    EQUALOP, 
    NOTEQUALOP, 
    GTHANOP, 
    LTHANOP, 
    GETHANOP, 
    LETHANOP,
    RPAREN,
    LPAREN,
    RBRACKET,
    LBRACKET,
    SEMICOLON,
    NEWLINE,
    PRINT,
    PREV,
    BEGIN, 
    EXPECT, 
    REVEAL,
    IF,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
}

pub struct Production {
    pub symbol: String,
    pub terminals: Vec<TokenType>,
    pub nonterminals: Vec<ConcattedProductions>,
}

pub struct ConcattedProductions {
    pub production: String,
    pub production1: String,
}

#[derive(Debug, Clone)]
pub struct CYKEntry {
    pub symbol: String,
    pub left_prev: Option<CYKBacktrack>, // store entry the index of table entry that lead to me so we can traverse it. None if terminal.
    pub right_prev: Option<CYKBacktrack>, // TODO: has a bug: I am going back to the whole previous index, not just the specific one that leds to me. Need to store index and symbol 
    pub token: Token,
}

#[derive(Debug, Clone)]
pub struct CYKBacktrack { // not the most optimal to do this 
    pub symbol: String, 
    pub index: (usize, usize),
}

impl FromStr for TokenType {
    type Err = ();
    fn from_str(input: &str) -> Result<TokenType, Self::Err> {
        match input {
            "NONE" => Ok(TokenType::NONE),
            "IDENTIFIER" => Ok(TokenType::IDENTIFIER("".to_string())),
            "ASSIGNMENT" => Ok(TokenType::ASSIGNMENT),
            "INTEGER" => Ok(TokenType::INTEGER(0)),
            "STRING" => Ok(TokenType::STRING("".to_string())),
            "FLOAT" => Ok(TokenType::FLOAT(0.0)),
            "ADDOP" => Ok(TokenType::ADDOP),
            "SUBOP" => Ok(TokenType::SUBOP),
            "MULOP" => Ok(TokenType::MULOP),
            "MODOP" => Ok(TokenType::MODOP),
            "DIVOP" => Ok(TokenType::DIVOP),
            "EQUALOP" => Ok(TokenType::EQUALOP), 
            "NOTEQUALOP" => Ok(TokenType::NOTEQUALOP),
            "GTHANOP" => Ok(TokenType::GTHANOP), 
            "LTHANOP" => Ok(TokenType::LTHANOP), 
            "GETHANOP" => Ok(TokenType::GETHANOP),
            "LETHANOP" => Ok(TokenType::LETHANOP), 
            "RPAREN" => Ok(TokenType::RPAREN),
            "LPAREN" => Ok(TokenType::LPAREN),
            "RBRACKET" => Ok(TokenType::RBRACKET),
            "LBRACKET" => Ok(TokenType::LBRACKET),
            "SEMICOLON" => Ok(TokenType::SEMICOLON),
            "NEWLINE" => Ok(TokenType::NEWLINE),
            "PRINT" => Ok(TokenType::PRINT),
            "PREV" => Ok(TokenType::PREV),
            "BEGIN" => Ok(TokenType::BEGIN),
            "REVEAL" => Ok(TokenType::REVEAL),
            "EXPECT" => Ok(TokenType::EXPECT),
            "IF" => Ok(TokenType::IF),
            _ => return Err(()),
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn ne(&self, other: &Self) -> bool {
        std::mem::discriminant(self) != std::mem::discriminant(other)
    }
}

impl Production {
    pub fn goes_to_terminal(&self, token: &Token) -> bool {
        for term in &self.terminals {
            if *term == token.token_type {
                return true;
            }
        }
        return false;
    }

    pub fn goes_to_nonterminal(&self, sym: &str, sym1: &str) -> bool {
        for prod_opt in &self.nonterminals {
            if prod_opt.production == sym.to_string() && prod_opt.production1 == sym1.to_string() {
                return true;
            }
        }
        return false;
    }
}
