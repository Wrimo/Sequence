use std::cell::RefCell;
use std::rc::Rc;

use super::parser::expr::{Expression, ExpressionType};
use super::parser::statement::{Program, Statement, StatementType};
use super::runtime_types::{History, HistoryCollection, Memory, VariableType};
use crate::interpreter::runtime_types::SharedHistory;
use crate::user_options::USER_OPTIONS;

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

pub fn calculate_expression(expr: Box<Expression>, memory: &mut Memory) -> VariableType {
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
            let history: SharedHistory = memory.get_history(s);
            let borrow = history.borrow();
            borrow.get_past(borrow.len() - 1).clone()
        }
        ExpressionType::PREV(s) => {
            let var_history: SharedHistory = memory.get_history(s);
            let borrow = var_history.borrow();
            if borrow.len() == 1 {
                return borrow.get_past(0).clone();
            }
            borrow.get_past(borrow.len() - 2).clone()
        }
        ExpressionType::ACCESSOR => {
            let name = expr.var_name.unwrap();
            let var_history: History = memory.get_history(name).borrow().clone();

            if !matches!(lhs, None) {
                // could clean this up with a simpler way to get values out of VariableType
                if let VariableType::INTEGER(x) = calculate_expression(lhs.unwrap(), memory).convert_int() {
                    return var_history.get_past(x as usize).clone();
                }
            } else if !matches!(rhs, None) {
                if let VariableType::INTEGER(x) = calculate_expression(rhs.unwrap(), memory).convert_int() {
                    return var_history.get_past(var_history.len() - 1 - (x as usize)).clone();
                }
            }
            return VariableType::INTEGER(0);
        }

        ExpressionType::LEN(s) => VariableType::INTEGER(memory.get_history(s).borrow().len() as i64),

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
    memory: &mut Memory,
) {
    for statement in program {
        if USER_OPTIONS.lock().unwrap().debug {
            println!("{:?}", statement.statement_type.clone());
        }

        match statement.statement_type.clone() {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), memory);
                let name: String = statement.var_name.clone().unwrap();

                memory.update_history(name, val);
            }

            StatementType::COPY => {
                let destination = statement.var_name.as_ref().unwrap().to_string();
                let source = statement.alt_var_name.as_ref().unwrap().to_string();

                memory.copy(source, destination);
            }

            StatementType::PRINT => {
                let exp = statement.expr.as_ref().unwrap();
                if matches!(exp.exp_type, ExpressionType::NONE) {
                    println!("");
                    continue;
                }
                let x = calculate_expression(statement.expr.clone().unwrap(), memory);
                print_variable(&x);

                for i in 0..statement.alt_exps.len() {
                    let x = calculate_expression(statement.alt_exps[i].clone(), memory);
                    print_variable(&x);
                }
                println!("");
            }

            StatementType::IF => {
                if calculate_expression(statement.expr.clone().unwrap(), memory).as_bool() {
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

                let var_history: SharedHistory = memory.get_history(statement.var_name.clone().unwrap());
                print!("{}: ", statement.var_name.clone().unwrap());
                for i in 0..var_history.borrow().len() {
                    print_variable(&var_history.borrow().get_past(i));
                }
                println!();
            }

            StatementType::RUN => {
                let mut shared_memory: Memory = Memory::new(); 
                for var in statement.var_list.as_ref().unwrap() { 
                    shared_memory.insert_history(var.clone(), memory.get_history(var.to_string()))
                }
                execute_program(statement.sub_program.as_ref().unwrap(), Some(shared_memory), &None); // TODO: replace shared memory with parameters
            }

            _ => {
                eprintln!("Bad statement {:?}", statement.statement_type);
            }
        }
    }
}

pub fn execute_program(program: &Program, shared_memory: Option<Memory>, parameters: &Option<HistoryCollection>) {
    let mut memory = match shared_memory {
        Some(x) => x, 
        None => Memory::new(),
    };

    if let Some(params) = parameters { // TODO: restructure there so there is an error if there are no parameters and some are expected 
        let expected_names = program.parameters.clone().expect("Program received unexpected parameters");
        if params.len() != expected_names.len() {
            panic!("{}: got {} parameters, expected {}", program.name, params.len(), expected_names.len());
        }

        for i in 0..expected_names.len() { 
            let shared_history = Rc::new(RefCell::new(params[i].clone())); 
            memory.insert_history(expected_names[i].clone(), shared_history);
        }
    }
    else { 
        panic!("{}: expected paramters, but none were given", program.name);
    }

    if USER_OPTIONS.lock().unwrap().debug { // probably should move this up so all programs are printed once, not once per run
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
            if calculate_expression(program.expect[i].expr.clone().unwrap(), &mut memory).as_bool() {
                run_statements(program.expect[i].code_block.as_ref().unwrap(), &mut memory);
                break 'prog_loop;
            }
        }
    }
}
