use std::error;

use crate::interpreter::code_types::Expression;

use super::parsing_types::{Token, TokenType};

pub struct Parser {
    current_token: Token,
    tokens: Vec<Token>,
    index: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current_token: tokens[0].clone(),
            tokens: tokens,
            index: 0,
        }
    }

    pub fn run(&mut self) {
        self.body();
    }

    fn error_missing_token(&self, t: TokenType) {
        eprintln!(
            "line {}: expected {:?} got {:?}",
            self.current_token.line, t, self.current_token.token_type
        );
        assert!(false);
    }

    fn error_custom(&self, msg: &str) {
        eprintln!("line {}: {}", self.current_token.line, msg);
        assert!(false);
    }

    fn next_token(&mut self) {
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.current_token = Token {
                token_type: TokenType::NONE,
                line: self.current_token.line,
            };
            return;
        }
        self.current_token = self.tokens[self.index].clone();
    }

    fn ahead(&self, i: usize) -> Token {
        self.tokens[self.index + i].clone()
    }

    fn accept(&mut self, t: &TokenType) -> bool {
        if self.current_token.equals(t) {
            self.next_token();
            return true;
        }
        return false;
    }

    fn expect(&mut self, t: TokenType) -> bool {
        if self.accept(&t) {
            return true;
        }
        self.error_missing_token(t);
        return false;
    }

    fn body(&mut self) {
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);
            if self.accept(&TokenType::NONE) {
                break;
            }
        }
    }

    fn code_block(&mut self) {
        self.expect(TokenType::LBRACKET);
        self.expect(TokenType::NEWLINE);
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);
            if self.accept(&TokenType::RBRACKET) {
                break;
            }
        }
    }

    fn expr(&mut self) {
        self.expr_comp();
        while self.accept(&TokenType::AND) || self.accept(&TokenType::OR) {
            self.expr_comp();
        }
    }

    fn expr_comp(&mut self) {
        self.epxr_add();
        while self.accept(&TokenType::GETHANOP)
            || self.accept(&TokenType::GTHANOP)
            || self.accept(&TokenType::EQUALOP)
            || self.accept(&TokenType::NOTEQUALOP)
            || self.accept(&TokenType::LTHANOP)
            || self.accept(&TokenType::LETHANOP)
        {
            self.epxr_add();
        }
    }

    fn epxr_add(&mut self) {
        if self.accept(&TokenType::ADDOP) || self.accept(&TokenType::SUBOP) {}
        self.expr_mul();
        while self.accept(&TokenType::ADDOP) || self.accept(&TokenType::SUBOP) {
            self.expr_mul();
        }
    }

    fn expr_mul(&mut self) {
        self.expr_expo();
        while self.accept(&TokenType::MULOP) || self.accept(&TokenType::DIVOP) {
            self.expr_expo();
        }
    }

    fn expr_expo(&mut self) {
        self.unary_fact();
        while self.accept(&TokenType::EXPONENT) {
            self.unary_fact();
        }
    }

    fn unary_fact(&mut self) {
        if self.accept(&TokenType::NOT) {}
        else if self.accept(&TokenType::FACTORIAL) {} 
        else if self.accept(&TokenType::SUBOP) {}
        self.factor(); 
    }
    
    fn factor(&mut self) {
        if self.accept(&TokenType::IDENTIFIER(String::from(""))) {
        } else if self.accept(&TokenType::INTEGER(0)) {
        } else if self.accept(&TokenType::FLOAT(0.0)) {
        } else if self.accept(&TokenType::TRUE) {
        } else if self.accept(&TokenType::FALSE) {
        } else if self.accept(&TokenType::LPAREN) {
            self.expr();
            self.expect(TokenType::RPAREN);
        } else {
            self.error_custom(format!("expression error for token {:?}", self.current_token).as_str());
        }
    }

    fn statement(&mut self) {
        if self.current_token.equals(&TokenType::IDENTIFIER(String::from(""))) && self.ahead(1).equals(&TokenType::ASSIGNMENT) {
            self.parse_stmt_assign();
        } else if self.accept(&TokenType::BEGIN) {
            self.parse_stmt_begin();
        } else if self.accept(&TokenType::EXPECT) {
            self.parse_stmt_expect();
        } else if self.accept(&TokenType::REVEAL) {
            self.parse_stmt_reveal();
        } else if self.accept(&TokenType::PRINT) {
            self.parse_stmt_print();
        } else if self.accept(&TokenType::IF) {
            self.parse_stmt_if();
        } else {
            self.error_custom("expected statement");
        }
    }

    fn parse_stmt_assign(&mut self) {
        self.expect(TokenType::IDENTIFIER(String::from("")));
        self.expect(TokenType::ASSIGNMENT);
        self.expr();
    }

    fn parse_stmt_begin(&mut self) {
        self.code_block();
    }

    fn parse_stmt_expect(&mut self) {
        self.expr();
        self.code_block();
    }

    fn parse_stmt_reveal(&mut self) {
        self.expect(TokenType::IDENTIFIER(String::from("")));
    }

    fn parse_stmt_print(&mut self) {
        // need to update to add multiple expressions to be printed
        self.expect(TokenType::LPAREN);
        self.expr();
        self.expect(TokenType::RPAREN);
    }

    fn parse_stmt_if(&mut self) {
        self.expr();
        self.code_block();

        while self.accept(&TokenType::ELIF) {
            self.expr();
            self.code_block();
        }

        if self.accept(&TokenType::ELSE) {
            self.code_block();
        }
    }

    // left to do for new parser
    // [x] booleans for expressions
    // [x] unary operators
    // [x] if/else statements
    // [] generate abstract syntax tree
}
