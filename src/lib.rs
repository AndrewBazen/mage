#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

pub mod parser;
pub mod interpreter;

use pest::Parser;
use crate::interpreter::interpret;

pub use crate::parser::{MageParser, Rule};

fn extract_shell_override(source: &str) -> Option<String> {
    if let Some(first_line) = source.lines().next() {
        if let Some(shell) = first_line.strip_prefix("#!shell:") {
            return Some(shell.trim().to_string());
        }
    }
    None
}

pub fn run(source: &str) -> Result<(), String> {
    let shell_override = extract_shell_override(source);
    let mut scope = HashMap::new();
    let mut functions = HashMap::new();
    let pairs = MageParser::parse(crate::Rule::program, source);
    match pairs {
        Ok(pairs) => {
            interpret(pairs, shell_override.as_deref(), &mut scope, &mut functions);
            Ok(())
        }
        Err(err) => Err(format!("Parse error: {}", err)),
    }
}
