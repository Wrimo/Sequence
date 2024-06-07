use super::expr::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    NONE,
    PRINT,
    REVEAL,
    ASSIGN,
    IF,
    ELSE,
    BEGIN,
    EXPECT,
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub var_name: Option<String>,
    pub code_block: Option<Vec<Statement>>,
    pub expr: Option<Box<Expression>>,
    pub alt_code_blocks: Vec<Vec<Statement>>,
    pub alt_exps: Vec<Box<Expression>>,
}

pub struct Program {
    pub begin: Option<Statement>,
    pub expect: Option<Statement>,
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program { 
        Program { 
            begin: None, 
            expect: None, 
            body: Vec::new(), 
        }
    }

    pub fn add(&mut self, s: Statement) { 
        self.body.push(s);
    }
}

impl StatementType {
    pub fn has_code_block(&self) -> bool {
        match self {
            StatementType::IF | StatementType::ELSE | StatementType::BEGIN | StatementType::EXPECT => true,
            _ => false,
        }
    }
}

impl Statement {
    pub fn new() -> Statement { 
        Statement { 
            statement_type: StatementType::NONE, 
            var_name: None, 
            code_block: None, 
            expr: None, 
            alt_code_blocks: Vec::new(), 
            alt_exps: Vec::new(), 
        }
    }
    pub fn set_type(&mut self, t: StatementType) { 
        self.statement_type = t; 
    }
    pub fn reset(&mut self) {
        self.statement_type = StatementType::NONE;
        self.var_name = None;
        self.expr = None;
        self.code_block = None;
    }
}
