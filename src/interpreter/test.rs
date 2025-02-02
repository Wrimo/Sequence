#[cfg(test)]
use std::process::{Command, Output};

#[cfg(test)]
fn assert_out(output: Output, expected: &str) {
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        String::from(expected) + &String::from(" \n"),
    );
}

#[test]
fn test_add_1_1() {
    let output = Command::new("target/debug/sequence")
        .args(["examples/add.sq", "{1}", "{1}"])
        .output()
        .unwrap();

    assert_out(output, "2");
}

#[test]
fn test_add_100_100() {
    let output = Command::new("target/debug/sequence")
        .args(["examples/add.sq", "{100}", "{100}"])
        .output()
        .unwrap();

    assert_out(output, "200");
}

#[test]
fn test_next_greater() {
    let output = Command::new("target/debug/sequence")
        .args(["examples/next_greater.sq", "{4, 5, 2, 4}", "{2}"])
        .output()
        .unwrap();

    assert_out(output, "4");
}
