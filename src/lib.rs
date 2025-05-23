#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

pub mod parser;
pub mod interpreter;
pub mod config;
pub mod bin;
pub mod syntax;

use pest::Parser;
use pest::iterators::Pairs;
use crate::interpreter::interpret;
use crate::config::MageConfig;

pub use crate::parser::{MageParser, Rule};

fn extract_shell_override(source: &str) -> Option<String> {
    if let Some(first_line) = source.lines().next() {
        if let Some(shell) = first_line.strip_prefix("#!shell:") {
            return Some(shell.trim().to_string());
        }
    }
    None
}

pub fn run(source: &str, cli_shell: Option<&str>) -> Result<(), String> {
    // Priority: 1. CLI shell override, 2. Script-defined shell, 3. Config file shell
    let script_shell = extract_shell_override(source);
    let config_shell = MageConfig::find_config().and_then(|c| c.shell);
    
    let shell_override = cli_shell
        .map(String::from)
        .or(script_shell)
        .or(config_shell);

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

pub fn format(source: &str) -> Result<String, String> {
    match MageParser::parse(crate::parser::Rule::program, source) {
        Ok(pairs) => {
            let mut result = String::new();

            for pair in pairs {
                let line = format_pair(pair);
                if !line.trim().is_empty() {
                    result.push_str(line.trim_end());
                    result.push('\n');
                }
            }
            Ok(result)
        }
        Err(err) => Err(format!("Parse error: {}", err)),
    }
}

fn format_pair(pair: pest::iterators::Pair<Rule>) -> String {
    match pair.as_rule() {
        Rule::conjure => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str();
            let value = inner.next().unwrap().as_str();
            format!("conjure {} = {}", ident, value)
        }
        Rule::incant => {
            let string = pair.into_inner().next().unwrap().as_str();
            format!("incant {}", string)
        }
        _ => pair.as_str().to_string(), // fallback for other rules
    }
}

pub fn parse_ast(source: &str) -> Result<Pairs<Rule>, String> {
    MageParser::parse(crate::Rule::program, source).map_err(|e| format!("Parse error: {}", e))
}

// Run REPL with optional shell override
pub fn run_repl(shell: Option<&str>) -> Result<(), String> {
    let config_shell = MageConfig::find_config().and_then(|c| c.shell);
    let final_shell = shell.map(String::from).or(config_shell);
    
    // Run the REPL implementation with shell override
    crate::bin::repl::run_repl(final_shell.as_deref())
}
