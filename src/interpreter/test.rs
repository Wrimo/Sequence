use std::collections::HashMap;
use std::process::Command;


// fn run_script(file: &str, args: Vec<&str>) {
//     let command = Command::new("cargo").ag
//              .args("../../examples" + file)
//              .spawn()
//              .expect(expected_output)
// }

#[test]
fn test_add_1_1() {
    Command::new("cargo")
             .args(["run",  "examples/add.sq", "{1}", "{1}"])
             .spawn()
             .expect("2");
}

#[test]
fn test_add_100_100() {
    Command::new("cargo")
             .args(["run",  "examples/add.sq", "{100}", "{100}"])
             .spawn()
             .expect("2");
}

#[test]
fn test_next_greater() {
    Command::new("cargo")
             .args(["run", "examples/next_greater.sq", "{4, 5, 2, 4}", "{2}"])
             .spawn()
             .expect("4");
}