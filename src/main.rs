use std::fs;
use mage::run;

fn main() {
    let path = std::env::args().nth(1).expect("🪄 Provide a .mage file to cast");
    let code = fs::read_to_string(&path).expect("📜 Failed to read spellbook");

    if let Err(e) = run(&code) {
        eprintln!("❌ {}", e);
        std::process::exit(1);
    }
}


