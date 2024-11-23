use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
            Self::INTEGER(_x) => {}
        }
        self.clone()
    }

    pub fn abs(&mut self) -> Self {
        self.bool_to_number();
        match self {
            VariableType::FLOAT(x) => {
                if *x < 0.0 {
                    VariableType::FLOAT(-1.0 * (*x))
                } else {
                    self.clone()
                }
            }
            VariableType::INTEGER(x) => {
                if *x < 0 {
                    VariableType::INTEGER(-1 * (*x))
                } else {
                    self.clone()
                }
            }
            _ => return self.clone(), // probably need to change this later
        }
    }
}

#[derive(Clone, Debug)]
pub struct History {
    items: Vec<VariableType>,
}

pub type HistoryCollection = Vec<SharedHistory>;
pub type SharedHistory = Rc<RefCell<History>>;

impl History {
    pub fn new() -> History { 
        History { 
            items: vec![],
        }
    }

    pub fn alloc(_name: String, val: VariableType) -> SharedHistory {
        Rc::new(RefCell::new(History {
            items: vec![val],
        }))
    }
    pub fn add(&mut self, val: VariableType) {
        self.items.push(val);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_past(&self, index: usize) -> VariableType {
        self.items[index].clone()
    }
}

pub struct Memory {
    pub cells: HashMap<String, SharedHistory>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { cells: HashMap::new() }
    }

    pub fn get_history(&self, name: String) -> SharedHistory {
        self.cells.get(&name).unwrap().clone()
    }

    pub fn update_history(&mut self, name: String, value: VariableType) {
        self.cells
            .entry(name.clone())
            .and_modify(|ent| (**ent).borrow_mut().add(value.clone()) )// (*ent).borrow_mut().add(value.clone()))
            .or_insert(History::alloc(name, value));
    }
 
    pub fn insert_history(&mut self, name: String, history: Rc<RefCell<History>>) {
        self.cells.insert(name, history);
    }

    pub fn copy(&mut self, source: String, destination: String) {
        let source_history: SharedHistory = self.get_history(source);

        self.cells
            .entry(destination)
            .and_modify(|ent| *ent = source_history.clone())
            .or_insert(source_history);
    }
}
