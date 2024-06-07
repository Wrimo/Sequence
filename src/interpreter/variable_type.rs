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