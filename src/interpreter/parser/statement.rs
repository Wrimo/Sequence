use super::expr::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    NONE,
    PRINT,
    REVEAL,
    ASSIGN,
    COPY,
    IF,
    BEGIN,
    EXPECT,
    RUN,
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub statement_type: StatementType,
    pub var_name: Option<String>,
    pub alt_var_name: Option<String>,
    pub var_list: Option<Vec<String>>,
    pub code_block: Option<Vec<Statement>>,
    pub expr: Option<Box<Expression>>,
    pub alt_code_blocks: Vec<Vec<Statement>>,
    pub alt_exps: Vec<Box<Expression>>,
    pub sub_program: Option<Box<Program>>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub begin: Option<Statement>,
    pub expect: Vec<Statement>,
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            begin: None,
            expect: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add(&mut self, s: Statement) {
        self.body.push(s);
    }
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            statement_type: StatementType::NONE,
            var_name: None,
            alt_var_name: None,
            var_list: None,
            code_block: None,
            expr: None,
            alt_code_blocks: Vec::new(),
            alt_exps: Vec::new(),
            sub_program: None,
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
        self.alt_code_blocks = Vec::new();
        self.alt_exps = Vec::new();
    }
}
