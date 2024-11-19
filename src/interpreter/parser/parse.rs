use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs, process};

use super::expr::{Expression, ExpressionType};
use super::lexer::symbol_analysis;
use super::parsing_types::{Token, TokenType};
use super::statement::{Program, Statement, StatementType};

pub struct Parser<'a> {
    current_token: Token,
    tokens: Vec<Token>,
    index: usize,
    prog: Program,
    stat: Statement,
    directory: PathBuf,
    prog_cache: &'a mut HashMap<String, Box<Program>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, prog_cache: &'a mut HashMap<String, Box<Program>>, file_path: &'a PathBuf) -> Parser<'a> {
        let mut directory = file_path.clone();
        let mut tokens = tokens.clone(); // TODO: don't make the parse require a newline at the end
        tokens.push(Token {
            token_type: TokenType::NEWLINE,
            line: 99999,
        });


        PathBuf::pop(&mut directory);
        Parser {
            current_token: tokens[0].clone(),
            tokens: tokens,
            index: 0,
            prog: Program::new(String::from(file_path.to_string_lossy())),
            stat: Statement::new(),
            directory: directory,
            prog_cache: prog_cache,
        }
    }

    pub fn run(&mut self) -> &Program {
        self.take();
        self.body();
        return &self.prog;
    }

    fn error_missing_token(&self, t: TokenType) {
        eprintln!(
            "line {}: expected {:?} got {:?}",
            self.current_token.line + 1,
            t,
            self.current_token.token_type
        );
        process::abort();
    }

    fn error_custom(&self, msg: &str) {
        eprintln!("line {}: {}", self.current_token.line + 1, msg);
        process::abort();
    }

    fn next_token(&mut self) -> Token {
        // moves to next token and returns previous
        self.index += 1;
        if self.index >= self.tokens.len() {
            self.current_token = Token {
                token_type: TokenType::NONE,
                line: self.current_token.line,
            };
            return self.current_token.clone();
        }
        self.current_token = self.tokens[self.index].clone();
        return self.tokens[self.index - 1].clone();
    }

    fn ahead(&self, i: usize) -> Token {
        self.tokens[self.index + i].clone()
    }

    fn accept(&mut self, t: TokenType) -> bool {
        if self.current_token.equals(t) {
            self.next_token();
            return true;
        }
        return false;
    }

    fn expect(&mut self, t: TokenType) -> bool {
        if self.accept(t.clone()) {
            return true;
        }
        self.error_missing_token(t);
        return false;
    }

    fn expect_identifier(&mut self) -> Option<String> {
        if self.current_token.equals(TokenType::IDENTIFIER(String::from(""))) {
            let t = self.current_token.clone();
            self.next_token();
            if let TokenType::IDENTIFIER(s) = t.token_type {
                return Some(s);
            }
            return None;
        }
        self.error_missing_token(TokenType::IDENTIFIER(String::from("")));
        None
    }

    fn take(&mut self) {
        if self.accept(TokenType::TAKE) {
            let mut params: Vec<String> = Vec::new();
            loop {
                params.push(self.expect_identifier().unwrap());

                if !self.accept(TokenType::COMMA) { 
                    break;
                }
            }
            self.expect(TokenType::NEWLINE);
            self.prog.parameters = Some(params);
        }
    }

    fn body(&mut self) {
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);

            match self.stat.statement_type {
                StatementType::BEGIN => self.prog.begin = Some(self.stat.clone()),
                StatementType::EXPECT => self.prog.expect.push(self.stat.clone()),
                _ => self.prog.add(self.stat.clone()),
            }
            self.stat.reset();

            if self.accept(TokenType::NONE) {
                break;
            }
        }
    }

    fn code_block(&mut self) -> Vec<Statement> {
        let old_stat = self.stat.clone();
        let mut code_block: Vec<Statement> = Vec::new();
        self.expect(TokenType::LBRACKET);
        self.expect(TokenType::NEWLINE);
        loop {
            self.statement();
            self.expect(TokenType::NEWLINE);
            code_block.push(self.stat.clone());
            self.stat.reset();
            if self.accept(TokenType::RBRACKET) {
                break;
            }
        }
        self.stat = old_stat;
        return code_block;
    }

    fn expr(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_comp();
        while self.current_token.equals(TokenType::AND) || self.current_token.equals(TokenType::OR) {
            let old = self.next_token();
            let rhs = self.expr_comp();
            match old.token_type {
                TokenType::AND => lhs = Expression::new(ExpressionType::AND, Some(lhs), Some(rhs)),
                TokenType::OR => lhs = Expression::new(ExpressionType::OR, Some(lhs), Some(rhs)),
                _ => {}
            }
        }
        return lhs;
    }

    fn expr_comp(&mut self) -> Box<Expression> {
        let mut lhs = self.epxr_add();
        while self.current_token.equals(TokenType::GETHANOP)
            || self.current_token.equals(TokenType::GTHANOP)
            || self.current_token.equals(TokenType::EQUALOP)
            || self.current_token.equals(TokenType::NOTEQUALOP)
            || self.current_token.equals(TokenType::LTHANOP)
            || self.current_token.equals(TokenType::LETHANOP)
        {
            let old = self.next_token();
            let rhs = self.epxr_add();
            match old.token_type {
                TokenType::GETHANOP => lhs = Expression::new(ExpressionType::GTHE, Some(lhs), Some(rhs)),
                TokenType::GTHANOP => lhs = Expression::new(ExpressionType::GTH, Some(lhs), Some(rhs)),
                TokenType::EQUALOP => lhs = Expression::new(ExpressionType::EQU, Some(lhs), Some(rhs)),
                TokenType::NOTEQUALOP => lhs = Expression::new(ExpressionType::NEQU, Some(lhs), Some(rhs)),
                TokenType::LTHANOP => lhs = Expression::new(ExpressionType::LTH, Some(lhs), Some(rhs)),
                TokenType::LETHANOP => lhs = Expression::new(ExpressionType::LTHE, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn epxr_add(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_mul();
        while self.current_token.equals(TokenType::ADDOP) || self.current_token.equals(TokenType::SUBOP) {
            let old = self.next_token();
            let rhs = self.expr_mul();

            match old.token_type {
                TokenType::ADDOP => lhs = Expression::new(ExpressionType::ADD, Some(lhs), Some(rhs)),
                TokenType::SUBOP => lhs = Expression::new(ExpressionType::SUB, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn expr_mul(&mut self) -> Box<Expression> {
        let mut lhs = self.expr_expo();
        while self.current_token.equals(TokenType::MULOP)
            || self.current_token.equals(TokenType::DIVOP)
            || self.current_token.equals(TokenType::MODOP)
        {
            let old = self.next_token();
            let rhs = self.expr_expo();
            match old.token_type {
                TokenType::MULOP => lhs = Expression::new(ExpressionType::MUL, Some(lhs), Some(rhs)),
                TokenType::DIVOP => lhs = Expression::new(ExpressionType::DIV, Some(lhs), Some(rhs)),
                TokenType::MODOP => lhs = Expression::new(ExpressionType::MOD, Some(lhs), Some(rhs)),

                _ => {}
            }
        }
        return lhs;
    }

    fn expr_expo(&mut self) -> Box<Expression> {
        let mut lhs = self.unary_fact();
        while self.accept(TokenType::EXPONENT) {
            let rhs = self.unary_fact();
            lhs = Expression::new(ExpressionType::EXPONENT, Some(lhs), Some(rhs));
        }
        return lhs;
    }

    fn unary_fact(&mut self) -> Box<Expression> {
        if self.accept(TokenType::NOT) {
            return Expression::new(ExpressionType::NOT, Some(self.factor()), None);
        } else if self.accept(TokenType::FACTORIAL) {
            return Expression::new(ExpressionType::FACTORIAL, Some(self.factor()), None);
        } else if self.accept(TokenType::SUBOP) {
            return Expression::new(ExpressionType::UMIN, Some(self.factor()), None);
        } else if self.accept(TokenType::VERTICALBAR) {
            return Expression::new(ExpressionType::ABS, Some(self.factor()), None);
        } else if self.accept(TokenType::PREV) {
            return Expression::new(ExpressionType::PREV(self.expect_identifier().unwrap()), None, None);
        } else if self.accept(TokenType::LEN) {
            let name: Option<String> = self.expect_identifier();
            return Expression::new(ExpressionType::LEN(name.unwrap()), None, None);
        }
        return self.accessor_factor();
    }

    fn accessor_factor(&mut self) -> Box<Expression> {
        let mut ident: Option<String> = None;
        let mut lhs: Option<Box<Expression>> = None;
        let mut rhs: Option<Box<Expression>> = None;
        if self.accept(TokenType::DOLLAR) {
            ident = self.expect_identifier();
        } else {
            lhs = Some(self.factor());
        }
        while self.accept(TokenType::ACCESSOR) {
            if self.accept(TokenType::DOLLAR) {
                if !matches!(ident.clone(), None) {
                    self.error_custom("multiple histories marked as source");
                }
                ident = self.expect_identifier();
            } else {
                rhs = Some(self.factor());
            }

            if matches!(ident, None) {
                self.error_custom("one side of must be marked an identifier marked as source ($a::1, i::$a, etc)");
            }
            let expr = Expression {
                exp_type: ExpressionType::ACCESSOR,
                lhs: lhs,
                rhs: rhs.clone(),
                var_name: ident.clone(),
            };
            lhs = Some(Box::new(expr));
        }
        return lhs.unwrap();
    }

    fn factor(&mut self) -> Box<Expression> {
        match self.next_token().token_type {
            TokenType::IDENTIFIER(s) => Expression::new(ExpressionType::IDENTIFIER(s), None, None),
            TokenType::INTEGER(x) => Expression::new(ExpressionType::INTEGER(x), None, None),
            TokenType::FLOAT(x) => Expression::new(ExpressionType::FLOAT(x), None, None),
            TokenType::TRUE => Expression::new(ExpressionType::BOOL(true), None, None),
            TokenType::FALSE => Expression::new(ExpressionType::BOOL(false), None, None),
            TokenType::STRING(x) => Expression::new(ExpressionType::STRING(x), None, None),

            TokenType::LPAREN => {
                let exp = self.expr();
                self.expect(TokenType::RPAREN);
                return exp;
            }

            _ => {
                self.error_custom(format!("expression error for token {:?}", self.current_token).as_str());
                return Expression::new(ExpressionType::NONE, None, None);
            }
        }
    }

    fn parse_string(&mut self) -> String {
        match self.current_token.token_type.clone() {
            TokenType::STRING(s) => {
                self.next_token();
                return s;
            }

            _ => {
                self.error_custom(format!("expected STRING, got {:?}", self.current_token).as_str());
                process::abort();
            }
        }
    }

    fn statement(&mut self) {
        self.stat.reset();
        if self.current_token.equals(TokenType::IDENTIFIER(String::from(""))) && self.ahead(1).equals(TokenType::ASSIGNMENT) {
            self.parse_stmt_assign();
        } else if self.current_token.equals(TokenType::IDENTIFIER(String::from(""))) && self.ahead(1).equals(TokenType::COPY) {
            self.parse_stmt_copy();
        } else if self.accept(TokenType::BEGIN) {
            self.parse_stmt_begin();
        } else if self.accept(TokenType::EXPECT) {
            self.parse_stmt_expect();
        } else if self.accept(TokenType::REVEAL) {
            self.parse_stmt_reveal();
        } else if self.accept(TokenType::PRINT) {
            self.parse_stmt_print();
        } else if self.accept(TokenType::IF) {
            self.parse_stmt_if();
        } else if self.accept(TokenType::RUN) {
            self.parse_stmt_call();
        }
    }

    fn parse_stmt_assign(&mut self) {
        self.stat.set_type(StatementType::ASSIGN);
        self.stat.var_name = self.expect_identifier();
        self.expect(TokenType::ASSIGNMENT);
        self.stat.expr = Some(self.expr());
    }

    fn parse_stmt_copy(&mut self) {
        self.stat.set_type(StatementType::COPY);
        self.stat.var_name = self.expect_identifier();
        self.expect(TokenType::COPY);
        self.stat.alt_var_name = self.expect_identifier();
    }

    fn parse_stmt_begin(&mut self) {
        self.stat.set_type(StatementType::BEGIN);
        self.stat.code_block = Some(self.code_block());
    }

    fn parse_stmt_expect(&mut self) {
        self.stat.set_type(StatementType::EXPECT);
        self.stat.expr = Some(self.expr());
        self.stat.code_block = Some(self.code_block())
    }

    fn parse_stmt_reveal(&mut self) {
        self.stat.set_type(StatementType::REVEAL);
        self.stat.var_name = self.expect_identifier();
    }

    fn parse_stmt_print(&mut self) {
        self.stat.set_type(StatementType::PRINT);
        self.expect(TokenType::LPAREN);

        if self.accept(TokenType::RPAREN) {
            // no given expression; print()
            self.stat.expr = Some(Expression::new(ExpressionType::NONE, None, None));
            return;
        }
        self.stat.expr = Some(self.expr());

        while self.accept(TokenType::COMMA) {
            let expr = self.expr();
            self.stat.alt_exps.push(expr);
        }

        self.expect(TokenType::RPAREN);
    }

    fn parse_stmt_if(&mut self) {
        self.stat.set_type(StatementType::IF);
        self.stat.expr = Some(self.expr());
        self.stat.code_block = Some(self.code_block());

        while self.accept(TokenType::ELIF) {
            let exp = self.expr();
            let block = self.code_block();
            self.stat.alt_exps.push(exp);
            self.stat.alt_code_blocks.push(block);
        }

        if self.accept(TokenType::ELSE) {
            let block = self.code_block();
            self.stat.alt_code_blocks.push(block);
        }
    }

    fn parse_stmt_call(&mut self) {
        let file_name = self.parse_string();
        self.stat.set_type(StatementType::RUN);

        let x = self.prog_cache.get(&file_name);

        if matches!(x, None) {
            let mut new_directory = self.directory.clone();
            PathBuf::push(&mut new_directory, &file_name);
            let buf = fs::read_to_string(&new_directory).unwrap_or_else(|_| {
                eprintln!("could not read file: {}", file_name);
                process::exit(1);
            });
            
            let mut p = Parser::new(symbol_analysis(&buf).unwrap(), self.prog_cache, &new_directory);
            let prog = Box::new(p.run().clone());
            self.stat.sub_program = Some(prog.clone());
            self.prog_cache.insert(file_name, prog);
        } else {
            self.stat.sub_program = Some((x.unwrap()).clone()); // cleaner way to write?
        }

        // need to add way to expose variables to other program
        if self.accept(TokenType::WITH) {
            let mut shared: Vec<String> = Vec::new();
            loop {
                let ident: String = self.expect_identifier().unwrap();
                shared.push(ident);
                
                if !self.accept(TokenType::COMMA) { 
                    break; 
                }
            }
            self.stat.var_list = Some(shared);
        }
    }
}
