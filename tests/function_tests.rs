use std::path::Path;
use std::process::Command;

fn run_mage(file: &str) -> String {
    let path = Path::new("tests/functions").join(file);

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
fn test_bestow_returns_value() {
    let output = run_mage("test_bestow.mage");
    // enchant double(x) { bestow x * 2 } -> cast double(5) should return 10
    assert!(output.contains("10"), "double(5) should return 10");
}

#[test]
fn test_function_call() {
    let output = run_mage("hello.mage");
    assert!(output.contains("Welcome, Mage"), "Should greet Mage");
    assert!(output.contains("Hello, Mage"), "Function should print greeting");
}
