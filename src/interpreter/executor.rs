use super::parser::expr::{Expression, ExpressionType};
use super::parser::lexer::symbol_analysis;
use super::parser::parse::Parser;
use super::parser::statement::{Statement, StatementType};
use super::variable_type::VariableType;
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;
use std::env::var;

macro_rules! perform_arth_op {
    ($x:ident, $y:ident, $memory:ident, $op:tt) => {
        match (&calculate_expression($x.unwrap(), $memory).bool_to_number(), &calculate_expression($y.unwrap(), $memory).bool_to_number()) {
            (VariableType::INTEGER(x), VariableType::INTEGER(y)) => VariableType::INTEGER((*x $op *y) as i64),
            (VariableType::FLOAT(x), VariableType::FLOAT(y)) => VariableType::FLOAT((*x $op *y)  as f64),
            (VariableType::FLOAT(x), VariableType::INTEGER(y)) => VariableType::FLOAT(*x $op (*y as f64) as f64),
            (VariableType::INTEGER(x), VariableType::FLOAT(y)) => VariableType::FLOAT(((*x as f64) $op *y) as f64),

            _ => {
                eprint!("Impossible operation");
                return VariableType::INTEGER(-1);
            }
    }
}
}

macro_rules! perform_comp_op {
    ($x:ident, $y:ident, $memory:ident, $op:tt) => {
        match (&calculate_expression($x.unwrap(), $memory).bool_to_number(), &calculate_expression($y.unwrap(), $memory).bool_to_number()) {
            (VariableType::INTEGER(x), VariableType::INTEGER(y)) => VariableType::BOOL(*x $op *y),
            (VariableType::FLOAT(x), VariableType::FLOAT(y)) => VariableType::BOOL(*x $op *y),
            (VariableType::FLOAT(x), VariableType::INTEGER(y)) => VariableType::BOOL(*x $op (*y as f64)),
            (VariableType::INTEGER(x), VariableType::FLOAT(y)) => VariableType::BOOL((*x as f64) $op *y),

            _ => {
                eprint!("Impossible operation");
                return VariableType::INTEGER(-1);
            }
    }
}
}

macro_rules! perform_log_op {
    ($x:ident, $y:ident, $memory:ident, $op:tt) => {
        VariableType::BOOL(
            calculate_expression($x.unwrap(), $memory).as_bool() $op calculate_expression($y.unwrap(), $memory).as_bool()
        )
    }
}

pub fn calculate_expression(expr: Box<Expression>, memory: &HashMap<String, Vec<VariableType>>) -> VariableType {
    let lhs = expr.lhs;
    let rhs = expr.rhs;
    match expr.exp_type {
        ExpressionType::ADD => perform_arth_op!(lhs, rhs, memory, +),
        ExpressionType::SUB => perform_arth_op!(lhs, rhs, memory, -),
        ExpressionType::MUL => perform_arth_op!(lhs, rhs, memory, *),
        ExpressionType::DIV => perform_arth_op!(lhs, rhs, memory, /),
        ExpressionType::MOD => perform_arth_op!(lhs, rhs, memory, %),
        ExpressionType::EQU => perform_comp_op!(lhs, rhs, memory, ==),
        ExpressionType::NEQU => perform_comp_op!(lhs, rhs, memory, !=),
        ExpressionType::LTH => perform_comp_op!(lhs, rhs, memory, <),
        ExpressionType::LTHE => perform_comp_op!(lhs, rhs, memory, <=),
        ExpressionType::GTH => perform_comp_op!(lhs, rhs, memory, >),
        ExpressionType::GTHE => perform_comp_op!(lhs, rhs, memory, >=),

        ExpressionType::AND => perform_log_op!(lhs, rhs, memory, &&),
        ExpressionType::OR => perform_log_op!(lhs, rhs, memory, ||),
        ExpressionType::NOT => calculate_expression(lhs.unwrap(), memory).negate(),

        ExpressionType::ABS => calculate_expression(lhs.unwrap(), memory).abs(),

        ExpressionType::FACTORIAL => {
            let x = calculate_expression(lhs.unwrap(), memory).convert_int();
            let mut product = 1;
            if let VariableType::INTEGER(val) = x {
                for i in 2..val + 1 {
                    product *= i;
                }
            }
            VariableType::INTEGER(product)
        }

        ExpressionType::EXPONENT => {
            let x = calculate_expression(lhs.unwrap(), memory).bool_to_number();
            let y = calculate_expression(rhs.unwrap(), memory).bool_to_number();

            match (x, y) {
                (VariableType::FLOAT(x), VariableType::FLOAT(y)) => VariableType::FLOAT(f64::powf(x, y)),
                (VariableType::FLOAT(x), VariableType::INTEGER(y)) => VariableType::FLOAT(f64::powf(x, y as f64)),
                (VariableType::INTEGER(x), VariableType::FLOAT(y)) => VariableType::FLOAT(f64::powf(x as f64, y)),
                (VariableType::INTEGER(x), VariableType::INTEGER(y)) => {
                    if y < 0 {
                        VariableType::FLOAT(f64::powf(x as f64, y as f64))
                    } else {
                        VariableType::INTEGER(x.pow(y.try_into().unwrap()))
                    }
                }
                (_, _) => VariableType::INTEGER(0),
            }
        }
        ExpressionType::INTEGER(x) => VariableType::INTEGER(x),
        ExpressionType::FLOAT(x) => VariableType::FLOAT(x),
        ExpressionType::BOOL(x) => VariableType::BOOL(x),

        ExpressionType::IDENTIFIER(s) => {
            let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
            var_history[var_history.len() - 1].clone()
        }
        ExpressionType::PREV(s) => {
            let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
            if var_history.len() == 1 {
                return var_history[0].clone();
            }
            var_history[var_history.len() - 2].clone()
        }
        ExpressionType::ACCESSOR => {
            if let ExpressionType::IDENTIFIER(s) = lhs.clone().unwrap().exp_type {
                let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
                // could clean this up with a simpler way to get values out of VariableType
                if let VariableType::INTEGER(x) = calculate_expression(rhs.unwrap(), &memory).convert_int() {
                    
                    return var_history[x as usize].clone();
                }
            } else if let ExpressionType::IDENTIFIER(s) = rhs.unwrap().exp_type {
                let var_history: &Vec<VariableType> = memory.get(&s).unwrap();
                if let VariableType::INTEGER(x) = calculate_expression(lhs.unwrap(), &memory).convert_int() {
                    return var_history[var_history.len() - (x as usize)].clone();
                }
            }
            return VariableType::INTEGER(0);
        }

        _ => VariableType::INTEGER(-1),
    }
}

fn print_variable(x: &VariableType) {
    match x {
        VariableType::BOOL(x) => print!("{} ", x),
        VariableType::INTEGER(x) => print!("{} ", x),
        VariableType::FLOAT(x) => print!("{} ", x),
        VariableType::STRING(x) => print!("{} ", x),
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
                let exp = statement.expr.as_ref().unwrap();
                if matches!(exp.exp_type, ExpressionType::NONE) {
                    println!("");
                    continue;
                }
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                print_variable(&x);

                for i in 0..statement.alt_exps.len() {
                    let x = calculate_expression(statement.alt_exps[i].clone(), &memory);
                    print_variable(&x);
                }
                println!("");
            }

            StatementType::IF => {
                if calculate_expression(statement.expr.clone().unwrap(), &memory).as_bool() {
                    execute_program(&statement.code_block.as_ref().unwrap(), memory);
                } else if statement.alt_code_blocks.len() != 0 {
                    for i in 0..statement.alt_code_blocks.len() {
                        if i >= statement.alt_exps.len() || calculate_expression(statement.alt_exps[i].clone(), memory).as_bool()
                        {
                            execute_program(&statement.alt_code_blocks[i], memory);
                            break;
                        }
                    }
                }
            }

            StatementType::REVEAL => {
                let var_history: &Vec<VariableType> = memory.get(&statement.var_name.clone().unwrap()).unwrap();
                for i in 0..var_history.len() {
                    print_variable(&var_history[i]);
                }
            }

            _ => {
                eprintln!("Bad statement {:?}", statement.statement_type);
            }
        }
    }
}

pub fn run_program(input: &str) {
    let tokens = symbol_analysis(input).unwrap(); // better errors later
    let mut parser = Parser::new(tokens);

    let program = parser.run();
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
    if let Some(begin) = &program.begin {
        execute_program(&begin.code_block.as_ref().unwrap(), &mut memory)
    }
    loop {
        execute_program(&program.body, &mut memory);
        // expect block logic
        if let Some(ref expect) = program.expect {
            if calculate_expression(expect.expr.clone().unwrap(), &memory).as_bool() {
                execute_program(&expect.code_block.as_ref().unwrap(), &mut memory);
                break;
            }
        }
    }
}
