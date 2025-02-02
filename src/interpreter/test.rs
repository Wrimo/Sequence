#[cfg(test)]
use std::process::Command;


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