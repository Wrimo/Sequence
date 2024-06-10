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
    ACCESSOR,
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

impl PartialEq for ExpressionType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }

    fn ne(&self, other: &Self) -> bool {
        std::mem::discriminant(self) != std::mem::discriminant(other)
    }
}
