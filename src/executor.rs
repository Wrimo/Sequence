use crate::code_types::{Expression, Program, Statement, StatementType, VariableType};
use crate::parser::parse;
use crate::program::generate_abstract_syntax;
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;

macro_rules! perform_op {
    ($x:ident, $y:ident, $memory:ident, $op:tt) => {
        match (&calculate_expression($x, $memory), &calculate_expression($y, $memory)) {
            (VariableType::INTEGER(x), VariableType::INTEGER(y)) => VariableType::INTEGER((*x $op *y) as i32),
            (VariableType::FLOAT(x), VariableType::FLOAT(y)) => VariableType::FLOAT((*x $op *y)  as f32),
            (VariableType::FLOAT(x), VariableType::INTEGER(y)) => VariableType::FLOAT(*x $op (*y as f32) as f32),
            (VariableType::INTEGER(x), VariableType::FLOAT(y)) => VariableType::FLOAT(((*x as f32) $op *y) as f32), 
            // could be cleaned up with the addition of a bool type

            _ => {
                eprint!("Impossible operation");
                return VariableType::INTEGER(-1);
            }
    }
}
}

fn calculate_expression(expr: Box<Expression>, memory: &HashMap<String, Vec<VariableType>>) -> VariableType {
    match *expr {
        Expression::ADD(x, y) => perform_op!(x, y, memory, +),
        Expression::SUB(x, y) => perform_op!(x, y, memory, -),
        Expression::MUL(x, y) => perform_op!(x, y, memory, *),
        Expression::DIV(x, y) => perform_op!(x, y, memory, /),
        // Expression::MOD(x, y) => perform_op!(x, y, memory, %),
        // Expression::EQU(x, y) => perform_op!(x, y, memory, ==),
        // Expression::NEQU(x, y) => perform_op!(x, y, memory, !=),
        // Expression::LTH(x, y) => perform_op!(x, y, memory, <),
        // Expression::LTHE(x, y) => perform_op!(x, y, memory, <=),
        // Expression::GTH(x, y) => perform_op!(x, y, memory, >),
        // Expression::GTHE(x, y) => perform_op!(x, y, memory, >=),

        Expression::INTEGER(x) => VariableType::INTEGER(x),
        Expression::FLOAT(x) => VariableType::FLOAT(x),

        Expression::IDENTIFIER(s) => {
            let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
            var_history[var_history.len() - 1].clone()
        }
        Expression::PREV(s) => {
            let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
            var_history[var_history.len() - 2].clone()
        }
        Expression::NONE | _ => {
            println!("bad expr {:?}", expr);
            VariableType::INTEGER(-5)
        }
    }
}

fn execute_program(program: &Vec<Statement>, memory: &mut HashMap<String, Vec<VariableType>>) {
    for statement in program {
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let name: String = statement.var_name.clone().unwrap();

                memory
                    .entry(name)
                    .and_modify(|ent| ent.push(val.clone()))
                    .or_insert(vec![val.clone()]);
            }
            StatementType::PRINT => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                println!("{:?}", x);
            }

            StatementType::IF => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                if x == VariableType::INTEGER(1) {
                    execute_program(&statement.code_block.as_ref().unwrap(), memory);
                } else if statement.alt_code_blocks.len() != 0 {
                    for i in 0..statement.alt_code_blocks.len() {
                        if i >= statement.alt_exps.len() || calculate_expression(statement.alt_exps[i].clone(), memory) == VariableType::INTEGER(1) {
                            execute_program(&statement.alt_code_blocks[i], memory);
                            break;
                        }
                    }
                }
            }

            StatementType::REVEAL => {
                let var_history: &Vec<VariableType> = memory.get(&statement.var_name.clone().unwrap()).unwrap();
                for i in 0..var_history.len() {
                    println!("{:?}", var_history[i]);
                }
            }

            _ => {
                eprintln!("Bad statement {:?}", statement.statement_type);
            }
        }
    }
}

pub fn run_program(input: &str) {
    let m = match parse(input) {
        Ok(x) => x,
        Err(()) => {
            println!("Parsing failed!");
            return;
        }
    };
    println!("Parsing succeeded!");

    let mut program: Program = Program {
        begin: None,
        expect: None,
        body: Vec::new(),
    };
    let mut body: Vec<Statement> = Vec::new();
    for ent in &m[0][m.len() - 1] {
        if ent.symbol == "<$S>" {
            let mut statement: Statement = Statement {
                statement_type: StatementType::NONE,
                var_name: None,
                expr: None,
                code_block: None,
                alt_code_blocks: Vec::new(),
                alt_exps: Vec::new(),
            };

            generate_abstract_syntax(Box::new(ent.clone()), &mut body, &mut program, &mut statement);
            program.body = body;
            break;
        }
    }
    let mut memory: HashMap<String, Vec<VariableType>> = HashMap::new();

    if USER_OPTIONS.lock().unwrap().debug {
        println!("program length: {}\n\n", program.body.len());
        for i in &program.body {
            println!("{:?}", i);
        }
    }

    if matches!(program.expect.as_ref(), None) {
        println!("WARNING: Running with no expect block, program will not terminate!");
    }
    if let Some(begin) = program.begin {
        execute_program(&begin.code_block.unwrap(), &mut memory)
    }
    loop {
        execute_program(&program.body, &mut memory);
        // expect block logic
        if let Some(ref expect) = program.expect {
            if calculate_expression(expect.expr.clone().unwrap(), &memory) == VariableType::INTEGER(1){
                execute_program(&expect.code_block.as_ref().unwrap(), &mut memory);
                break;
            }
        }
    }
}
