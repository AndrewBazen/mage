pub mod repl {
    use std::collections::HashMap;
    use crate::parser::MageParser;
    use crate::interpreter::interpret;
    use pest::Parser;
    use crate::Rule;
    use rustyline::{error::ReadlineError, Editor};
    use rustyline::config::Config;
    use rustyline::completion::{Completer, Pair};
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::Validator;
    use rustyline::Helper;
    use std::borrow::Cow;

    struct MageCompleter;

    impl Completer for MageCompleter {
        type Candidate = Pair;

        fn complete(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) 
            -> Result<(usize, Vec<Pair>), ReadlineError> {
            let keywords = [
                "conjure",
                "incant",
                "curse",
                "evoke",
                "if",
                "loop",
                "cast",
                "enchant",
                "exit",
                "quit",
                "help",
                "clear"
            ];
            let start = line[..pos].rfind(|c: char| c.is_whitespace()).map_or(0, |i| i + 1);
            let word = &line[start..pos];
            let matches = keywords.iter()
                .filter(|&&k| k.starts_with(word))
                .map(|kw| Pair {
                    display: kw.to_string(),
                    replacement: kw.to_string(),
                })  
                .collect();
            Ok((start, matches))
        }
    }

    impl Highlighter for MageCompleter {
        fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
            let keywords = [
                "conjure",
                "incant",
                "curse",
                "evoke",
                "if",
                "loop",
                "cast",
                "enchant",
                "exit",
                "quit",
                "help",
                "clear"
            ];
            let mut out = String::new();
            for word in line.split_whitespace() {
                if keywords.contains(&word) {
                    out.push_str(&format!("\x1b[34m{}\x1b[0m ", word));
                } else {
                    out.push_str(word);
                    out.push(' ');
                }
            }
            Cow::Owned(out)
        }
    }

    // Required empty implementations for Helper trait
    impl Hinter for MageCompleter {
        type Hint = String;
        
        fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<String> {
            None
        }
    }

    impl Validator for MageCompleter {
        fn validate(&self, _ctx: &mut rustyline::validate::ValidationContext) -> rustyline::Result<rustyline::validate::ValidationResult> {
            Ok(rustyline::validate::ValidationResult::Valid(None))
        }
    }

    impl Helper for MageCompleter {}

    pub fn run_repl(shell_override: Option<&str>) -> Result<(), String> {
        println!("🧙 Welcome to Mage REPL. Type 'exit' to quit.");
        
        if let Some(shell) = shell_override {
            println!("🪄 Using shell: {}", shell);
        }
        
        let mut scope = HashMap::new();
        let mut functions = HashMap::new();
        
        // Setup rustyline with our MageCompleter
        let config = Config::builder()
            .auto_add_history(true)
            .build();
        let completer = MageCompleter;
        let mut rl = match Editor::with_config(config) {
            Ok(editor) => editor,
            Err(err) => {
                return Err(format!("Error initializing rustyline: {}", err));
            }
        };
        rl.set_helper(Some(completer));
        
        loop {
            match rl.readline("mage> ") {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed == "exit" || trimmed == "quit" {
                        break;
                    }
                    if !trimmed.is_empty() {
                        // Intentionally leak the string to make it 'static 
                        // This is a memory leak, but it's acceptable for a REPL
                        // where the program exits when the REPL ends
                        let input: &'static str = Box::leak(trimmed.to_string().into_boxed_str());
                        
                        match MageParser::parse(Rule::program, input) {
                            Ok(pairs) => interpret(pairs, shell_override, &mut scope, &mut functions),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                },
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }
        
        Ok(())
    }
} 