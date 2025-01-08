use std::cell::RefCell;
use std::rc::Rc;

use super::parser::expr::{Expression, ExpressionType, HistoryExpression, HistoryExpressionType};
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
                (VariableType::FLOAT(x), VariableType::FLOAT(y)) => {
                    VariableType::FLOAT(f64::powf(x, y))
                }
                (VariableType::FLOAT(x), VariableType::INTEGER(y)) => {
                    VariableType::FLOAT(f64::powf(x, y as f64))
                }
                (VariableType::INTEGER(x), VariableType::FLOAT(y)) => {
                    VariableType::FLOAT(f64::powf(x as f64, y))
                }
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
        ExpressionType::STRING(x) => VariableType::STRING(x),

        ExpressionType::IDENTIFIER(s) => {
            let history: SharedHistory = memory.get_history(s);
            let borrow = history.borrow();
            borrow.get_past(borrow.len() - 1).clone()
        }

        ExpressionType::LEN(s) => {
            VariableType::INTEGER(memory.get_history(s).borrow().len() as i64)
        }

        _ => {
            eprintln!("recevied bad expression type");
            panic!();
        }
    }
}

fn evalulate_history_expression(memory: &Memory, expr: Box<HistoryExpression>) -> SharedHistory {
    match expr.exp_type {
        HistoryExpressionType::IDENTIFIER(s) => memory.get_history(s), // TODO need to allocate new history when this happens, how was I handling this in the past ? 

        HistoryExpressionType::PREV => {
            let history: SharedHistory = evalulate_history_expression(memory, expr.lhs.unwrap());
            let borrowed = history.borrow();
            let mut value: Option<VariableType> = None; 
            
            if borrowed.len() == 1 { 
                value = Some(borrowed.get_past(0).clone()); // todo - expensive if past value is a history, expected here
            } else {
                value = Some(borrowed.get_past(borrowed.len() - 2).clone());
            }

            match value.clone().unwrap() {
                VariableType::History(x) => x, 
                _ => panic!("Value {:?} does not have a previous value", value.unwrap()),
            }
        }

        // ExpressionType::ACCESSOR => {
        //     let name = expr.var_name.unwrap();
        //     let var_history: History = memory.get_history(name).borrow().clone(); // todo - want to avoid this type of copy

        //     if !matches!(lhs, None) {
        //         // could clean this up with a simpler way to get values out of VariableType
        //         if let VariableType::INTEGER(x) =
        //             calculate_expression(lhs.unwrap(), memory).convert_int()
        //         {
        //             return var_history.get_past(x as usize).clone();
        //         }
        //     } else if !matches!(rhs, None) {
        //         if let VariableType::INTEGER(x) =
        //             calculate_expression(rhs.unwrap(), memory).convert_int()
        //         {
        //             return var_history
        //                 .get_past(var_history.len() - 1 - (x as usize))
        //                 .clone();
        //         }
        //     }
        //     return VariableType::INTEGER(0);
        // }
        _ => panic!(),
    }
}

fn get_printable_history(x: SharedHistory) -> String {
    let mut values: Vec<String> = Vec::new();
    for i in 0..x.borrow().len() {
        values.push(get_printable_value(&x.borrow().get_past(i)));
    }

    let value_text = values.join(", ");
    return format!("[{}]", value_text);
}

fn get_printable_value(x: &VariableType) -> String {
    match x {
        VariableType::BOOL(x) => format!("{}", x),
        VariableType::INTEGER(x) => format!("{}", x),
        VariableType::FLOAT(x) => format!("{}", x),
        VariableType::STRING(x) => format!("{}", x),
        VariableType::History(x) => get_printable_history(x.clone()),
    }
}

fn run_statements(program: &Vec<Statement>, memory: &mut Memory) {
    for statement in program {
        if USER_OPTIONS.lock().unwrap().debug {
            println!("{:?}", statement.statement_type.clone());
        }

        match statement.statement_type.clone() {
            // TODO: split statement execution into different function
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), memory);
                let history: SharedHistory = evalulate_history_expression(memory, statement.history_expr.clone().unwrap());
                history.borrow_mut().add(val);
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

                // todo - could get rid of this is if i moved the first expr to the same list as the others
                let x = calculate_expression(statement.expr.clone().unwrap(), memory);
                println!("{}", get_printable_value(&x));

                for i in 0..statement.alt_exps.len() {
                    let x = calculate_expression(statement.alt_exps[i].clone(), memory);
                    println!("{}", get_printable_value(&x));
                }
                println!("");
            }

            StatementType::IF => {
                if calculate_expression(statement.expr.clone().unwrap(), memory).as_bool() {
                    run_statements(&statement.code_block.as_ref().unwrap(), memory);
                } else if statement.alt_code_blocks.len() != 0 {
                    for i in 0..statement.alt_code_blocks.len() {
                        if i >= statement.alt_exps.len()
                            || calculate_expression(statement.alt_exps[i].clone(), memory).as_bool()
                        {
                            run_statements(&statement.alt_code_blocks[i], memory);
                            break;
                        }
                    }
                }
            }

            StatementType::REVEAL => {
                let var_history: SharedHistory =
                    memory.get_history(statement.var_name.clone().unwrap());
                print!("{}: ", statement.var_name.clone().unwrap());
                print!("{}", get_printable_history(var_history));
                println!();
            }

            StatementType::RUN => {
                // for each variable insert a shared insert history with that name in current memory

                let sub_prog: Box<Program> = statement.sub_program.as_ref().unwrap().clone();
                let mut parameters: Option<HistoryCollection> = None;

                if let Some(parameter_names) = sub_prog.parameters {
                    let given_histories = statement.var_list.as_ref().unwrap();
                    let mut new_parameters: HistoryCollection = HistoryCollection::new();
                    assert_eq!(given_histories.len(), parameter_names.len());

                    for (current_name, _) in given_histories.iter().zip(parameter_names.iter()) {
                        let hist: Rc<RefCell<History>> = memory.get_history(current_name.clone());
                        new_parameters.push(hist.clone());
                    }

                    parameters = Some(new_parameters);
                }

                execute_program(statement.sub_program.as_ref().unwrap(), None, parameters);
                // TODO: replace shared memory with parameters
            }

            _ => {
                eprintln!("Bad statement {:?}", statement.statement_type);
            }
        }
    }
}

pub fn execute_program(
    program: &Program,
    shared_memory: Option<Memory>,
    parameters: Option<HistoryCollection>,
) {
    let mut memory = match shared_memory {
        Some(x) => x,
        None => Memory::new(),
    };

    // todo - should clean up this function, especially move parmeter logic to another function
    if let Some(params) = parameters {
        let expected_names = program
            .parameters
            .clone()
            .expect("Program received unexpected parameters");
        if params.len() != expected_names.len() {
            panic!(
                "{}: got {} parameters, expected {}",
                program.name,
                params.len(),
                expected_names.len()
            );
        }

        for i in 0..expected_names.len() {
            let shared_history = params[i].clone();
            memory.insert_history(expected_names[i].clone(), shared_history);
        }
    } else if matches!(program.parameters, Some(_)) {
        panic!("{}: expected parameters, but none were given", program.name)
    }

    if USER_OPTIONS.lock().unwrap().debug {
        // probably should move this up so all programs are printed once, not once per run
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
            if calculate_expression(program.expect[i].expr.clone().unwrap(), &mut memory).as_bool()
            {
                run_statements(program.expect[i].code_block.as_ref().unwrap(), &mut memory);
                break 'prog_loop;
            }
        }
    }
}
