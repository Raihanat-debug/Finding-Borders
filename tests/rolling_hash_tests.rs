use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_sample() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "rolling_hash"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rolling_hash");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"abcababcab\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "2 5");
}

#[test]
fn test_no_border() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "rolling_hash"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rolling_hash");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"abcdef\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(stdout.trim(), "");
}