#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    NONE,
    PRINT,
    ASSIGN,
    IF
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub var_name: Option<String>,
    pub code_block: Option<Vec<Statement>>, 
    pub expr: Option<Box<Expression>>,
}

#[derive(Clone, Debug)]
pub enum Expression { 
    ADD(Box<Expression>, Box<Expression>), 
    SUB(Box<Expression>, Box<Expression>),
    MUL(Box<Expression>, Box<Expression>), 
    DIV(Box<Expression>, Box<Expression>), 
    MOD(Box<Expression>, Box<Expression>), 
    PREV(String), 
    IDENTIFIER(String),
    INTEGER(i32), 
    NONE
}

impl Statement {
    pub fn reset(&mut self) {
        self.statement_type = StatementType::NONE;
        self.var_name = None;
        self.expr = None;
        self.code_block = None;  
    }
}
