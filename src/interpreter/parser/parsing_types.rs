use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum TokenType {
    NONE,
    IDENTIFIER(String),
    ASSIGNMENT,
    INTEGER(i64),
    STRING(String),
    FLOAT(f64),
    TRUE,
    FALSE,
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
    VERTICALBAR,
    NEWLINE,
    PRINT,
    PREV,
    BEGIN,
    EXPECT,
    REVEAL,
    IF,
    ELSE,
    ELIF, 
    AND,
    OR,
    NOT,
    FACTORIAL,
    EXPONENT,
    ABS,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize, 
}

#[derive(Debug, Clone)]
pub struct CYKEntry {
    pub symbol: String,
    pub left_prev: Option<Box<CYKEntry>>, // store entry the index of table entry that lead to me so we can traverse it. None if terminal.
    pub right_prev: Option<Box<CYKEntry>>,
    pub token: Token,
}

impl FromStr for TokenType {
    type Err = ();
    fn from_str(input: &str) -> Result<TokenType, Self::Err> {
        match input {
            "NONE" => Ok(TokenType::NONE),
            "IDENTIFIER" => Ok(TokenType::IDENTIFIER("".to_string())),
            "ASSIGNMENT" => Ok(TokenType::ASSIGNMENT),
            "INTEGER" => Ok(TokenType::INTEGER(0)),
            "TRUE" => Ok(TokenType::TRUE),
            "FALSE" => Ok(TokenType::FALSE),
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
            "ELSE" => Ok(TokenType::ELSE), 
            "ELIF" => Ok(TokenType::ELIF), 
            "AND" => Ok(TokenType::AND), 
            "OR" => Ok(TokenType::OR), 
            "NOT" => Ok(TokenType::NOT),
            "ABS" => Ok(TokenType::ABS), 
            "FACTORIAL" => Ok(TokenType::FACTORIAL),
            "EXPONENT" => Ok(TokenType::EXPONENT),
            "VERTICALBAR" => Ok(TokenType::VERTICALBAR),
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

impl PartialEq for Token { 
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }

    fn ne(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl Token { 
    pub fn equals(&self, other: &TokenType) -> bool { 
        self.token_type == *other
    }
}