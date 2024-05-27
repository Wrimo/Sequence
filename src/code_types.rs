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

impl StatementType {
    pub fn has_code_block(&self) -> bool {
        match self {
            StatementType::IF | StatementType::ELSE | StatementType::BEGIN | StatementType::EXPECT => true,
            _ => false,
        }
    }
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

#[derive(Clone, Debug)]
pub enum Expression {
    ADD(Box<Expression>, Box<Expression>),
    SUB(Box<Expression>, Box<Expression>),
    MUL(Box<Expression>, Box<Expression>),
    DIV(Box<Expression>, Box<Expression>),
    MOD(Box<Expression>, Box<Expression>),
    EQU(Box<Expression>, Box<Expression>),
    NEQU(Box<Expression>, Box<Expression>),
    GTH(Box<Expression>, Box<Expression>),
    GTHE(Box<Expression>, Box<Expression>),
    LTH(Box<Expression>, Box<Expression>),
    LTHE(Box<Expression>, Box<Expression>),
    PREV(String),
    IDENTIFIER(String),
    INTEGER(i32),
    NONE,
}

impl Statement {
    pub fn reset(&mut self) {
        self.statement_type = StatementType::NONE;
        self.var_name = None;
        self.expr = None;
        self.code_block = None;
    }
}
