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
    AND(Box<Expression>, Box<Expression>),
    OR(Box<Expression>, Box<Expression>),
    NOT(Box<Expression>),
    FACTORIAL(Box<Expression>),
    EXPONENT(Box<Expression>, Box<Expression>),
    ABS(Box<Expression>),
    PREV(String),
    IDENTIFIER(String),
    BOOL(bool),
    INTEGER(i64),
    FLOAT(f64),
    NONE,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VariableType {
    FLOAT(f64),
    INTEGER(i64),
    BOOL(bool),
    STRING(String),
}

impl VariableType {
    pub fn as_bool(&self) -> bool {
        match self {
            Self::FLOAT(x) => *x >= 1.0,
            Self::INTEGER(x) => *x >= 1,
            Self::BOOL(x) => *x,
            Self::STRING(x) => *x != "".to_string(),
        }
    }

    pub fn bool_to_number(&mut self) -> Self {
        // if is a bool, converts it to an integer for expression eval
        match self {
            Self::BOOL(x) => *self = Self::INTEGER(if *x { 1 } else { 0 }),
            _ => {}
        }
        self.clone()
    }

    pub fn negate(&self) -> Self {
        VariableType::BOOL(!self.as_bool())
    }

    pub fn convert_int(&mut self) -> Self { 
        match self { 
            Self::BOOL(x) => *self = Self::INTEGER(*x as i64), 
            Self::FLOAT(x) => *self = Self::INTEGER(*x as i64), 
            Self::STRING(_x) => *self = Self::INTEGER(0), // neeed to change later
            Self::INTEGER(_x) => {},
        }
        self.clone()
    }

    pub fn abs(&mut self) -> Self { 
        self.bool_to_number(); 
        match self { 
            VariableType::FLOAT(x) => if *x < 0.0 { VariableType::FLOAT(-1.0 * (*x))} else {self.clone()}, 
            VariableType::INTEGER(x) => if *x < 0 { VariableType::INTEGER(-1 * (*x))} else {self.clone()}, 
            _ => {return self.clone()}, // probably need to change this later
        }
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
    pub fn reset(&mut self) {
        self.statement_type = StatementType::NONE;
        self.var_name = None;
        self.expr = None;
        self.code_block = None;
    }
}
