#[derive(Clone, Debug, PartialEq)]
pub enum StatementType {
    NONE,
    PRINT,
    ASSIGN,
}


#[derive(Clone)]
pub struct Statement {
    pub statement_type: StatementType,
    pub var_name: Option<String>,
    pub val: Option<i32>, // replace with expression
}

impl Statement {
    pub fn reset(&mut self) { 
        self.statement_type = StatementType::NONE;
        self.var_name = None; 
        self.val = None;
    }
}
