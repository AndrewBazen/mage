use std::fs;
use std::path::Path;
use std::process::Command;

/// Run a single .mage file and check it executes without error
fn run_mage_file(path: &Path) -> Result<(), String> {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        return Err(format!(
            "Script failed with exit code {:?}\nSTDOUT:\n{}\nSTDERR:\n{}",
            output.status.code(),
            stdout,
            stderr
        ));
    }

    Ok(())
}

#[test]
fn test_all_mage_scripts() {
    let tests_dir = Path::new("tests");
    let mut failures: Vec<(String, String)> = Vec::new();
    let mut passed = 0;

    // Skip these files (temporary files, platform-specific, etc.)
    let skip_files = [
        "tmp_hello.mage",
        "unix_test.mage", // Unix-specific
    ];

    for entry in fs::read_dir(tests_dir).expect("Failed to read tests directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "mage") {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            // Skip certain files
            if skip_files.contains(&file_name) {
                println!("Skipping: {}", file_name);
                continue;
            }

            print!("Testing: {} ... ", file_name);

            match run_mage_file(&path) {
                Ok(()) => {
                    println!("OK");
                    passed += 1;
                }
                Err(e) => {
                    println!("FAILED");
                    failures.push((file_name.to_string(), e));
                }
            }
        }
    }

    println!("\n{} passed, {} failed", passed, failures.len());

    if !failures.is_empty() {
        println!("\nFailures:");
        for (name, error) in &failures {
            println!("\n=== {} ===\n{}", name, error);
        }
        panic!("{} test(s) failed", failures.len());
    }
}

#[test]
fn test_hello_mage_script() {
    // Write a temporary .mage script
    let script = "conjure name = \"Mage\";\n\
incant \"Hello, $name!\";\n\
##\n# This is a multi-line comment\n# Another line\n##\n\
incant \"Bye, ${name}!\";\n";
    let script_path = "tests/tmp_hello.mage";
    fs::write(script_path, script).unwrap();

    // Run the mage interpreter
    let output = Command::new("cargo")
        .args(["run", "--", script_path])
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
    println!("Exit code: {:?}", output.status.code());
    assert!(stdout.contains("Hello, Mage!"));
    assert!(stdout.contains("Bye, Mage!"));

    // Clean up
    let _ = fs::remove_file(script_path);
}
