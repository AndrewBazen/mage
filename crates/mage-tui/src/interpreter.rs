use std::collections::HashMap;
use std::sync::mpsc;

use mage_core::Rule;
use mage_core::interpreter::{ExprValue, FunctionDef, interpret};
use mage_core::output::OutputCollector;
use mage_core::parser::MageParser;
use pest::Parser;

#[derive(Debug, Clone)]
pub struct CommandResult {
    pub command: String,
    pub stdout_lines: Vec<String>,
    pub stderr_lines: Vec<String>,
    pub success: bool,
}

/// Runs on a dedicated std::thread. Owns scope and functions (non-Send types stay here).
/// Receives commands via cmd_rx, sends results via result_tx.
pub fn interpreter_thread(cmd_rx: mpsc::Receiver<String>, result_tx: mpsc::Sender<CommandResult>) {
    let mut scope: HashMap<String, ExprValue> = HashMap::new();
    let mut functions: HashMap<String, FunctionDef<'static>> = HashMap::new();

    while let Ok(command) = cmd_rx.recv() {
        // Leak input string to get 'static lifetime (same pattern as CLI REPL)
        let input: &'static str = Box::leak(command.clone().into_boxed_str());

        let mut collector = OutputCollector::buffered();
        let success = match MageParser::parse(Rule::program, input) {
            Ok(pairs) => match interpret(pairs, None, &mut scope, &mut functions, &mut collector) {
                Ok(()) => true,
                Err(e) => {
                    collector.eprintln(&format!("{}", e));
                    false
                }
            },
            Err(e) => {
                collector.eprintln(&format!("Parse error: {}", e));
                false
            }
        };

        let stdout_lines = collector.take_stdout();
        let stderr_lines = collector.take_stderr();

        let _ = result_tx.send(CommandResult {
            command,
            stdout_lines,
            stderr_lines,
            success,
        });
    }
}
