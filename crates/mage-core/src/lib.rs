#[macro_use]
extern crate pest_derive;

use std::collections::HashMap;

pub mod builtins;
pub mod config;
pub mod interpreter;
pub mod output;
pub mod package;
pub mod parser;

use crate::config::MageConfig;
use crate::interpreter::{ExprValue, interpret};
use crate::output::OutputCollector;
use pest::Parser;
use pest::iterators::Pairs;

pub use crate::interpreter::{ExprValue as Value, FunctionDef};
pub use crate::output::{InterpreterError, OutputCollector as Output};
pub use crate::parser::{MageParser, Rule};

/// Extract shell override from script source (e.g., `#!shell:bash`)
fn extract_shell_override(source: &str) -> Option<String> {
    if let Some(first_line) = source.lines().next()
        && let Some(shell) = first_line.strip_prefix("#!shell:")
    {
        return Some(shell.trim().to_string());
    }
    None
}

/// Run mage source code with optional shell override
pub fn run(source: &str, cli_shell: Option<&str>) -> Result<(), String> {
    let script_shell = extract_shell_override(source);
    let config_shell = MageConfig::find_config().and_then(|c| c.shell);

    let shell_override = cli_shell
        .map(String::from)
        .or(script_shell)
        .or(config_shell);

    let mut scope: HashMap<String, ExprValue> = HashMap::new();
    let mut functions = HashMap::new();
    let mut output = OutputCollector::direct();
    let pairs = MageParser::parse(crate::Rule::program, source);
    match pairs {
        Ok(pairs) => interpret(
            pairs,
            shell_override.as_deref(),
            &mut scope,
            &mut functions,
            &mut output,
        )
        .map_err(|e| format!("{}", e)),
        Err(err) => Err(format!("Parse error: {}", err)),
    }
}

/// Format mage source code
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
        _ => pair.as_str().to_string(),
    }
}

/// Parse mage source into AST
pub fn parse_ast(source: &str) -> Result<Pairs<'_, Rule>, String> {
    MageParser::parse(crate::Rule::program, source).map_err(|e| format!("Parse error: {}", e))
}
