use std::process::Command;
use std::str;

#[test]
fn test_mage_setup_dry_run_executes() {
    let output = Command::new("cargo")
        .args(["run", "--", "setup", "--dry-run"])
        .output()
        .expect("Failed to run mage setup --dry-run");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let stderr = str::from_utf8(&output.stderr).unwrap();

    println!("stdout:\n{}", stdout);
    println!("stderr:\n{}", stderr);

    assert!(output.status.success(), "Setup command failed");
    assert!(
        stdout.contains("dry-run"),
        "Output did not mention dry-run mode"
    );
    assert!(
        stdout.contains("Mage setup complete"),
        "Output did not indicate setup finished"
    );
}
