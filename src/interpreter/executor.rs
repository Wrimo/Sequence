use super::parser::expr::{Expression, ExpressionType};
use super::parser::lexer::symbol_analysis;
use super::parser::parse::Parser;
use super::parser::statement::{Program, Statement, StatementType};
use super::variable_type::VariableType;
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;

macro_rules! perform_arth_op {
    ($x:ident, $y:ident, $memory:ident, $op:tt) => {
        match (&calculate_expression($x, $memory).bool_to_number(), &calculate_expression($y, $memory).bool_to_number()) {
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
        match (&calculate_expression($x, $memory).bool_to_number(), &calculate_expression($y, $memory).bool_to_number()) {
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
            calculate_expression($x, $memory).as_bool() $op calculate_expression($y, $memory).as_bool()
        )
    }
}

pub fn calculate_expression(expr: Box<Expression>, memory: &HashMap<String, Vec<VariableType>>) -> VariableType {
    match expr.exp_type {
        // ExpressionType::ADD(x, y) => perform_arth_op!(x, y, memory, +),
        // ExpressionType::SUB(x, y) => perform_arth_op!(x, y, memory, -),
        // ExpressionType::MUL(x, y) => perform_arth_op!(x, y, memory, *),
        // ExpressionType::DIV(x, y) => perform_arth_op!(x, y, memory, /),
        // ExpressionType::MOD(x, y) => perform_arth_op!(x, y, memory, %),
        // ExpressionType::EQU(x, y) => perform_comp_op!(x, y, memory, ==),
        // ExpressionType::NEQU(x, y) => perform_comp_op!(x, y, memory, !=),
        // ExpressionType::LTH(x, y) => perform_comp_op!(x, y, memory, <),
        // ExpressionType::LTHE(x, y) => perform_comp_op!(x, y, memory, <=),
        // ExpressionType::GTH(x, y) => perform_comp_op!(x, y, memory, >),
        // ExpressionType::GTHE(x, y) => perform_comp_op!(x, y, memory, >=),

        // ExpressionType::AND(x, y) => perform_log_op!(x, y, memory, &&),
        // ExpressionType::OR(x, y) => perform_log_op!(x, y, memory, ||),
        // ExpressionType::NOT(x) => calculate_expression(x, memory).negate(),

        // ExpressionType::ABS(x) => calculate_expression(x, memory).abs(),

        // ExpressionType::FACTORIAL(x) => {
        //     let x = calculate_expression(x, memory).convert_int();
        //     let mut product = 1;
        //     if let VariableType::INTEGER(val) = x {
        //         for i in 2..val + 1 {
        //             product *= i;
        //         }
        //     }
        //     VariableType::INTEGER(product)
        // }

        // ExpressionType::EXPONENT(x, y) => {
        //     let x = calculate_expression(x, memory).bool_to_number();
        //     let y = calculate_expression(y, memory).bool_to_number();

        //     match (x, y) {
        //         (VariableType::FLOAT(x), VariableType::FLOAT(y)) => VariableType::FLOAT(f64::powf(x, y)),
        //         (VariableType::FLOAT(x), VariableType::INTEGER(y)) => VariableType::FLOAT(f64::powf(x, y as f64)),
        //         (VariableType::INTEGER(x), VariableType::FLOAT(y)) => VariableType::FLOAT(f64::powf(x as f64, y)),
        //         (VariableType::INTEGER(x), VariableType::INTEGER(y)) => {
        //             if y < 0 {
        //                 VariableType::FLOAT(f64::powf(x as f64, y as f64))
        //             } else {
        //                 VariableType::INTEGER(x.pow(y.try_into().unwrap()))
        //             }
        //         }
        //         (_, _) => VariableType::INTEGER(0),
        //     }
        // }
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
        _ | ExpressionType::NONE => {
            println!("bad expr {:?}", expr);
            VariableType::INTEGER(-5)
        }
    }
}

fn print_variable(x: &VariableType) {
    match x {
        VariableType::BOOL(x) => println!("{}", x),
        VariableType::INTEGER(x) => println!("{}", x),
        VariableType::FLOAT(x) => println!("{}", x),
        VariableType::STRING(x) => println!("{}", x),
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
                print_variable(&x);
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

    // if matches!(program.expect.as_ref(), None) {
    //     println!("WARNING: Running with no expect block, program will not terminate!");
    // }
    // if let Some(begin) = &program.begin {
    //     execute_program(&begin.code_block.as_ref().unwrap(), &mut memory)
    // }
    // loop {
    //     execute_program(&program.body, &mut memory);
    //     // expect block logic
    //     if let Some(ref expect) = program.expect {
    //         if calculate_expression(expect.expr.clone().unwrap(), &memory).as_bool() {
    //             execute_program(&expect.code_block.as_ref().unwrap(), &mut memory);
    //             break;
    //         }
    //     }
    // }
}
