use std::path::Path;
use std::process::Command;

fn run_mage(file: &str) -> String {
    let path = Path::new("tests/loops").join(file);

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
fn test_channel_loop() {
    let output = run_mage("channel_test.mage");
    // Channel loops from 0 to 9
    for i in 0..10 {
        assert!(output.contains(&i.to_string()), "Expected {} in output", i);
    }
}

#[test]
fn test_clean_channel() {
    let output = run_mage("clean_channel_test.mage");
    assert!(output.contains("0"));
    assert!(output.contains("1"));
    assert!(output.contains("2"));
}

#[test]
fn test_minimal_channel() {
    let output = run_mage("minimal_test.mage");
    assert!(output.contains("0"), "Expected 0 in output");
}

#[test]
fn test_dispel_breaks_loop() {
    let output = run_mage("test_dispel_loop.mage");
    // Loop should print 0-4 then dispel at 5
    assert!(output.contains("0"));
    assert!(output.contains("4"));
    assert!(!output.contains("5"), "dispel should break before 5");
}

#[test]
fn test_semicolon_optional() {
    let output = run_mage("semicolon_test.mage");
    assert!(output.contains("0"));
}

#[test]
fn test_oneline_syntax() {
    let output = run_mage("oneline_test.mage");
    assert!(output.contains("0"));
}

#[test]
fn test_format() {
    let output = run_mage("format_test.mage");
    assert!(output.contains("0"));
}

#[test]
fn test_debug_channel() {
    let output = run_mage("debug_channel.mage");
    assert!(!output.is_empty());
}

#[test]
fn test_debug_channel_loop() {
    let output = run_mage("debug_channel_loop.mage");
    assert!(!output.is_empty());
}

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn test_unix_specific() {
    let output = run_mage("unix_test.mage");
    assert!(!output.is_empty());
}
