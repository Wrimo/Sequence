#[derive(Clone, Debug)]

pub enum ExpressionType {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    EQU,
    NEQU,
    GTH,
    GTHE,
    LTH,
    LTHE,
    AND,
    OR,
    NOT,
    FACTORIAL,
    EXPONENT,
    UMIN, 
    ABS,
    PREV(String),
    IDENTIFIER(String),
    BOOL(bool),
    INTEGER(i64),
    FLOAT(f64),
    NONE,
}

#[derive(Clone, Debug)]
pub struct Expression { 
    pub exp_type: ExpressionType, 
    pub lhs: Option<Box<Expression>>, 
    pub rhs: Option<Box<Expression>>, 
}

impl Expression { 
    pub fn new(epx_type: ExpressionType, lhs: Option<Box<Expression>>, rhs: Option<Box<Expression>>) -> Box<Expression> { 
        Box::new(Expression { 
            exp_type: epx_type, 
            lhs: lhs, 
            rhs: rhs, 
        })
    }
}