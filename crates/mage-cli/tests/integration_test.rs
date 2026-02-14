use std::fs;
use std::process::Command;

#[test]
fn test_hello_mage_script() {
    // Write a temporary .mage script
    let script = "conjure name = \"Mage\";\n\
incant \"Hello, $name!\";\n\
##\n# This is a multi-line comment\n# Another line\n##\n\
incant \"Bye, ${name}!\";\n";
    let script_path = "tmp_hello.mage";
    fs::write(script_path, script).unwrap();

    // Run the mage interpreter
    let output = Command::new("cargo")
        .args(["run", "-p", "mage-cli", "--", script_path])
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
