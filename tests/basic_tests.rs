use std::path::Path;
use std::process::Command;

fn run_mage(file: &str) -> String {
    let path = Path::new("tests/basics").join(file);

    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", path.to_str().unwrap()])
        .output()
        .expect("Failed to execute mage");

    assert!(
        output.status.success(),
        "Script '{}' failed with exit code {:?}\nSTDERR:\n{}",
        file,
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn test_minimal_working() {
    let output = run_mage("minimal_working_test.mage");
    // Script should run without error
    let _ = output;
}

#[test]
fn test_simple_debug() {
    // This test just verifies the script runs without error
    // It only creates variables, no output expected
    let _ = run_mage("simple_debug.mage");
}

#[test]
fn test_newline_handling() {
    let output = run_mage("newline_test.mage");
    assert!(!output.is_empty());
}
