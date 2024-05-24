use crate::code_types::{Expression, Program, Statement, StatementType};
use crate::parser::parse;
use crate::program::generate_abstract_syntax;
use crate::user_options::USER_OPTIONS;
use std::collections::HashMap;

fn calculate_expression(expr: Box<Expression>, memory: &HashMap<String, Vec<i32>>) -> i32 {
    match *expr {
        Expression::ADD(x, y) => calculate_expression(x, memory) + calculate_expression(y, memory),
        Expression::SUB(x, y) => calculate_expression(x, memory) - calculate_expression(y, memory),
        Expression::MUL(x, y) => calculate_expression(x, memory) * calculate_expression(y, memory),
        Expression::DIV(x, y) => calculate_expression(x, memory) / calculate_expression(y, memory),
        Expression::MOD(x, y) => calculate_expression(x, memory) % calculate_expression(y, memory),
        Expression::EQU(x, y) => (calculate_expression(x, memory) == calculate_expression(y, memory)) as i32,
        Expression::NEQU(x, y) => (calculate_expression(x, memory) != calculate_expression(y, memory)) as i32,
        Expression::LTH(x, y) => (calculate_expression(x, memory) < calculate_expression(y, memory)) as i32,
        Expression::LTHE(x, y) => (calculate_expression(x, memory) <= calculate_expression(y, memory)) as i32,
        Expression::GTH(x, y) => (calculate_expression(x, memory) > calculate_expression(y, memory)) as i32,
        Expression::GTHE(x, y) => (calculate_expression(x, memory) >= calculate_expression(y, memory)) as i32,

        Expression::INTEGER(x) => x,
        Expression::IDENTIFIER(s) => {
            let var_history: &Vec<i32> = memory.get(&s).unwrap();
            var_history[var_history.len() - 1]
        }

        Expression::PREV(s) => {
            let var_history: &Vec<i32> = memory.get(&s).unwrap();
            var_history[var_history.len() - 2]
        }

        Expression::NONE => 0,
    }
}

fn execute_program(program: &Vec<Statement>, memory: &mut HashMap<String, Vec<i32>>) {
    for statement in program {
        match statement.statement_type {
            StatementType::ASSIGN => {
                let val = calculate_expression(statement.expr.clone().unwrap(), &memory);
                let name: String = statement.var_name.clone().unwrap();
                memory.entry(name).and_modify(|ent| ent.push(val)).or_insert(vec![val]);
            }
            StatementType::PRINT => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                println!("{}", x);
            }

            StatementType::IF => {
                let x = calculate_expression(statement.expr.clone().unwrap(), &memory);
                if x >= 1 {
                    execute_program(&statement.code_block.as_ref().unwrap(), memory);
                }
            }

            StatementType::REVEAL => {
                let var_history: &Vec<i32> = memory.get(&statement.var_name.clone().unwrap()).unwrap();
                for i in 0..var_history.len() {
                    println!("{}", var_history[i]);
                }
            }

            _ => {}
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
            };

            generate_abstract_syntax(Box::new(ent.clone()), &mut body, &mut program, &mut statement);
            program.body = body;
            break;
        }
    }
    let mut memory: HashMap<String, Vec<i32>> = HashMap::new();

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
            if calculate_expression(expect.expr.clone().unwrap(), &memory) == 1 {
                execute_program(&expect.code_block.as_ref().unwrap(), &mut memory);
                break;
            }
        }
    }
}
