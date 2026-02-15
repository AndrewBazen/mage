use std::path::Path;
use std::process::Command;

fn run_mage(file: &str) -> String {
    let path = Path::new("tests/conditionals").join(file);

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
fn test_simple_scry() {
    let output = run_mage("simple_scry_test.mage");
    assert!(output.contains("Scry works!"), "scry block should execute when condition is true");
    assert!(output.contains("Basic scry test complete"));
}

#[test]
fn test_scry_morph_lest() {
    let output = run_mage("scry_morph_lest_test.mage");
    // First scry: test_var == "hello" -> should print "scry printed"
    // Second scry: test_var != "goodbye", but morph test_var == "hello" -> "morph printed"
    assert!(output.contains("scry printed"), "First scry should match");
    assert!(output.contains("morph printed"), "Second morph should match");
}

#[test]
fn test_scry_morph() {
    let output = run_mage("scry_morph_test.mage");
    assert!(output.contains("This should print"), "Scry condition should execute");
}

#[test]
fn test_if_else() {
    let output = run_mage("if_else_test.mage");
    assert!(output.contains("Condition is true"), "Scry block should execute");
    assert!(output.contains("Scry-lest test complete"));
}

#[test]
fn test_arcane_syntax() {
    let output = run_mage("arcane_syntax_test.mage");
    assert!(output.contains("novice mage"), "Should identify as novice");
    assert!(output.contains("few spells"), "Should have few spells");
    assert!(output.contains("Arcane syntax test complete"));
}

#[test]
fn test_config_demo() {
    let output = run_mage("config_demo.mage");
    assert!(output.contains("Config variable is set"));
}

#[test]
fn test_block_debug() {
    // This test just verifies the script runs without error
    let _ = run_mage("block_debug.mage");
}

#[test]
fn test_debug_scry() {
    let output = run_mage("debug_scry.mage");
    assert!(!output.is_empty());
}

#[test]
fn test_debug_simple_block() {
    let output = run_mage("debug_simple_block.mage");
    assert!(!output.is_empty());
}

#[test]
fn test_debug_detailed() {
    let output = run_mage("debug_detailed.mage");
    assert!(!output.is_empty());
}

#[test]
fn test_invoke_seal() {
    let output = run_mage("invoke_seal_test.mage");
    // Error should be caught with summon
    assert!(output.contains("Before error"), "Should print before error");
    assert!(output.contains("Caught error: Something went wrong!"), "Should catch and print error");
    assert!(!output.contains("After error"), "Should not print after summon");
    // No error case
    assert!(output.contains("No error here"));
    assert!(!output.contains("This should not print"), "Seal block should not run without error");
    // Summon with expression
    assert!(output.contains("Caught: Error code: 42"), "Should catch expression error");
    assert!(output.contains("Tests complete!"));
}
