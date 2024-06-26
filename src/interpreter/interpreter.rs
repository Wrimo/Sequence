use super::parser::expr::{Expression, ExpressionType};
use super::parser::lexer::symbol_analysis;
use super::parser::parse::Parser;
use super::parser::statement::{self, Program, Statement, StatementType};
use super::variable_type::VariableType;
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;
use std::{fs, process};

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
            let name = expr.var_name.unwrap();

            if !matches!(lhs, None) {
                let var_history: &Vec<VariableType> = memory.get(&name).unwrap();
                // could clean this up with a simpler way to get values out of VariableType
                if let VariableType::INTEGER(x) = calculate_expression(lhs.unwrap(), &memory).convert_int() {
                    return var_history[x as usize].clone();
                }
            } else if !matches!(rhs, None) {
                let var_history: &Vec<VariableType> = memory.get(&name).unwrap();
                if let VariableType::INTEGER(x) = calculate_expression(rhs.unwrap(), &memory).convert_int() {
                    return var_history[var_history.len() - 1 - (x as usize)].clone();
                }
            }
            return VariableType::INTEGER(0);
        }

        ExpressionType::LEN(s) => VariableType::INTEGER(memory.get(&s).unwrap().len() as i64),

        _ => VariableType::INTEGER(-1), // should do something else here!
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

fn run_statements(
    program: &Vec<Statement>,
    memory: &mut HashMap<String, Vec<VariableType>>,
) {
    for statement in program {
        if USER_OPTIONS.lock().unwrap().debug {
            println!("{:?}", statement.statement_type.clone());
        }

        match statement.statement_type.clone() {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let name: String = statement.var_name.clone().unwrap();

                memory
                    .entry(name)
                    .and_modify(|ent| ent.push(val.clone()))
                    .or_insert(vec![val.clone()]);
            }

            StatementType::COPY => {
                let destination = statement.var_name.as_ref().unwrap().to_string();
                let source = statement.alt_var_name.as_ref().unwrap().to_string();

                let history: Vec<VariableType> = memory.get(&source).unwrap().clone();

                memory
                    .entry(destination)
                    .and_modify(|ent| *ent = history.clone())
                    .or_insert(history);
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
                    run_statements(&statement.code_block.as_ref().unwrap(), memory);
                } else if statement.alt_code_blocks.len() != 0 {
                    for i in 0..statement.alt_code_blocks.len() {
                        if i >= statement.alt_exps.len() || calculate_expression(statement.alt_exps[i].clone(), memory).as_bool()
                        {
                            run_statements(&statement.alt_code_blocks[i], memory);
                            break;
                        }
                    }
                }
            }

            StatementType::REVEAL => {
                let var_history: &Vec<VariableType> = memory.get(&statement.var_name.clone().unwrap()).unwrap();
                print!("{}: ", statement.var_name.clone().unwrap());
                for i in 0..var_history.len() {
                    print_variable(&var_history[i]);
                }
                println!();
            }

            StatementType::RUN => {
                execute_program(statement.sub_program.as_ref().unwrap());
            }

            _ => {
                eprintln!("Bad statement {:?}", statement.statement_type);
            }
        }
    }
}

pub fn execute_program(program: &Program) {
    let mut memory: HashMap<String, Vec<VariableType>> = HashMap::new(); // probably a good idea to rewrite this as a struct with its own functions

    if USER_OPTIONS.lock().unwrap().debug {
        println!("{:?}", program.begin);
        println!("{:?}", program.expect);
        println!("program length: {}\n\n", program.body.len());
        for i in &program.body {
            println!("{:?}", i);
        }
    }

    if program.expect.len() == 0 {
        println!("WARNING: Running with no expect block, program will not terminate!");
    }
    if let Some(begin) = &program.begin {
        run_statements(&begin.code_block.as_ref().unwrap(), &mut memory);
    }
    'prog_loop: loop {
        run_statements(&program.body, &mut memory);
        // expect block logic
        for i in 0..program.expect.len() {
            if calculate_expression(program.expect[i].expr.clone().unwrap(), &memory).as_bool() {
                run_statements(program.expect[i].code_block.as_ref().unwrap(), &mut memory);
                break 'prog_loop;
            }
        }
    }
}
