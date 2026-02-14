pub mod repl {
    use mage_core::Rule;
    use mage_core::interpreter::{ExprValue, FunctionDef, interpret};
    use mage_core::parser::MageParser;
    use crate::syntax;
    use pest::Parser;
    use rustyline::Helper;
    use rustyline::completion::{Completer, Pair};
    use rustyline::config::Config;
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::validate::Validator;
    use rustyline::{Editor, error::ReadlineError};
    use std::borrow::Cow;
    use std::collections::HashMap;

    struct MageCompleter {
        syntax_colors: syntax::TerminalColors,
        tree_sitter_available: bool,
    }

    impl MageCompleter {
        fn new() -> Self {
            let tree_sitter_available = syntax::is_tree_sitter_available();

            MageCompleter {
                syntax_colors: syntax::TerminalColors::new(),
                tree_sitter_available,
            }
        }
    }

    impl Completer for MageCompleter {
        type Candidate = Pair;

        fn complete(
            &self,
            line: &str,
            pos: usize,
            _ctx: &rustyline::Context<'_>,
        ) -> Result<(usize, Vec<Pair>), ReadlineError> {
            let keywords = [
                "conjure", "incant", "curse", "evoke", "if", "loop", "chant", "cast", "enchant",
                "exit", "quit", "help", "clear",
            ];
            let start = line[..pos]
                .rfind(|c: char| c.is_whitespace())
                .map_or(0, |i| i + 1);
            let word = &line[start..pos];
            let matches = keywords
                .iter()
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
            // Try to use tree-sitter highlighting if available
            if self.tree_sitter_available
                && let Some(tree) = syntax::parse(line)
            {
                // A very simple tree-sitter based highlighter
                let mut result = String::new();
                let colors = &self.syntax_colors;

                // Function to process a node and add it to the result with highlighting
                fn highlight_node(
                    node: &tree_sitter::Node,
                    source: &str,
                    result: &mut String,
                    colors: &syntax::TerminalColors,
                ) {
                    let node_text = &source[node.start_byte()..node.end_byte()];

                    // Apply colors based on node type
                    match node.kind() {
                        "variable_declaration" => {
                            // Find the keyword and identifier
                            if let Some(keyword) = node.child(0) {
                                result.push_str(colors.get_color("keyword"));
                                result.push_str(&source[keyword.start_byte()..keyword.end_byte()]);
                                result.push_str(syntax::TerminalColors::reset());
                                result.push(' ');

                                if let Some(name) = node.child_by_field_name("name") {
                                    result.push_str(colors.get_color("variable.declaration"));
                                    result.push_str(&source[name.start_byte()..name.end_byte()]);
                                    result.push_str(syntax::TerminalColors::reset());

                                    // Add the rest of the node
                                    for i in 2..node.child_count() {
                                        if let Some(child) = node.child(i) {
                                            // Skip operator for now
                                            if child.kind() == "=" {
                                                result.push_str(" = ");
                                            } else {
                                                highlight_node(&child, source, result, colors);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        "string" => {
                            result.push_str(colors.get_color("string"));
                            result.push_str(node_text);
                            result.push_str(syntax::TerminalColors::reset());
                        }
                        "number" => {
                            result.push_str(colors.get_color("number"));
                            result.push_str(node_text);
                            result.push_str(syntax::TerminalColors::reset());
                        }
                        "comment" | "multiline_comment" => {
                            result.push_str(colors.get_color("comment"));
                            result.push_str(node_text);
                            result.push_str(syntax::TerminalColors::reset());
                        }
                        "function_declaration" | "function_call" => {
                            if let Some(keyword) = node.child(0) {
                                result.push_str(colors.get_color("keyword"));
                                result.push_str(&source[keyword.start_byte()..keyword.end_byte()]);
                                result.push_str(syntax::TerminalColors::reset());
                                result.push(' ');

                                if let Some(name) = node.child_by_field_name("name") {
                                    result.push_str(colors.get_color("function"));
                                    result.push_str(&source[name.start_byte()..name.end_byte()]);
                                    result.push_str(syntax::TerminalColors::reset());
                                }

                                // Add the rest without colorization
                                for i in 2..node.child_count() {
                                    if let Some(child) = node.child(i) {
                                        if child.kind() == "string"
                                            || child.kind() == "number"
                                            || child.kind() == "comment"
                                            || child.kind() == "multiline_comment"
                                        {
                                            highlight_node(&child, source, result, colors);
                                        } else {
                                            result.push_str(
                                                &source[child.start_byte()..child.end_byte()],
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        "output" | "error" | "command" => {
                            if let Some(keyword) = node.child(0) {
                                result.push_str(colors.get_color("keyword"));
                                result.push_str(&source[keyword.start_byte()..keyword.end_byte()]);
                                result.push_str(syntax::TerminalColors::reset());
                                result.push(' ');

                                // Add the rest with appropriate colors
                                for i in 1..node.child_count() {
                                    if let Some(child) = node.child(i) {
                                        highlight_node(&child, source, result, colors);
                                    }
                                }
                            }
                        }
                        "if_statement" | "loop_statement" => {
                            if let Some(keyword) = node.child(0) {
                                result.push_str(colors.get_color("keyword"));
                                result.push_str(&source[keyword.start_byte()..keyword.end_byte()]);
                                result.push_str(syntax::TerminalColors::reset());

                                // Add the rest with appropriate colors
                                for i in 1..node.child_count() {
                                    if let Some(child) = node.child(i) {
                                        highlight_node(&child, source, result, colors);
                                    }
                                }
                            }
                        }
                        _ => {
                            // Process children for complex nodes
                            if node.child_count() > 0 {
                                result.push_str(node_text);
                            } else {
                                for i in 0..node.child_count() {
                                    if let Some(child) = node.child(i) {
                                        highlight_node(&child, source, result, colors);
                                    }
                                }
                            }
                        }
                    }
                }

                let root = tree.root_node();
                highlight_node(&root, line, &mut result, colors);

                if result.is_empty() {
                    return Cow::Owned(result);
                }
            }

            // Fallback to basic keyword highlighting
            let keywords = [
                "conjure", "incant", "curse", "evoke", "scry", "morph", "lest", "chant", "recite",
                "channel", "loop", "enchant", "cast", "yield",
            ];

            // Apply simple syntax highlighting with colors
            let colors = &self.syntax_colors;
            let mut out = String::new();
            let mut buffer = String::new();
            let mut in_string = false;
            let mut in_comment = false;

            for (i, c) in line.chars().enumerate() {
                if in_comment {
                    buffer.push(c);
                    if c == '\n' {
                        out.push_str(colors.get_color("comment"));
                        out.push_str(&buffer);
                        out.push_str(syntax::TerminalColors::reset());
                        buffer.clear();
                        in_comment = false;
                    }
                } else if in_string {
                    buffer.push(c);
                    if c == '"' && line.chars().nth(i.saturating_sub(1)) != Some('\\') {
                        out.push_str(colors.get_color("string"));
                        out.push_str(&buffer);
                        out.push_str(syntax::TerminalColors::reset());
                        buffer.clear();
                        in_string = false;
                    }
                } else if c == '"' {
                    if !buffer.is_empty() {
                        let word_ended = buffer
                            .chars()
                            .last()
                            .is_none_or(|last| last.is_whitespace());

                        if word_ended {
                            // Check for keyword at the end of the buffer
                            let last_word = buffer.split_whitespace().last().unwrap_or("");
                            if keywords.contains(&last_word) {
                                let prefix = &buffer[..buffer.len() - last_word.len()];
                                out.push_str(prefix);
                                out.push_str(colors.get_color("keyword"));
                                out.push_str(last_word);
                                out.push_str(syntax::TerminalColors::reset());
                            } else {
                                out.push_str(&buffer);
                            }
                        } else {
                            out.push_str(&buffer);
                        }
                        buffer.clear();
                    }

                    buffer.push(c);
                    in_string = true;
                } else if c == '#' {
                    // Found start of comment
                    if !buffer.is_empty() {
                        out.push_str(&buffer);
                        buffer.clear();
                    }
                    buffer.push(c);
                    in_comment = true;
                } else if c.is_whitespace() {
                    // End of a word
                    if keywords.contains(&buffer.as_str()) {
                        out.push_str(colors.get_color("keyword"));
                        out.push_str(&buffer);
                        out.push_str(syntax::TerminalColors::reset());
                    } else {
                        out.push_str(&buffer);
                    }
                    buffer.clear();
                    out.push(c);
                } else {
                    buffer.push(c);
                }
            }

            // Handle any remaining buffer
            if !buffer.is_empty() {
                if in_string {
                    out.push_str(colors.get_color("string"));
                    out.push_str(&buffer);
                    out.push_str(syntax::TerminalColors::reset());
                } else if in_comment {
                    out.push_str(colors.get_color("comment"));
                    out.push_str(&buffer);
                    out.push_str(syntax::TerminalColors::reset());
                } else if keywords.contains(&buffer.as_str()) {
                    out.push_str(colors.get_color("keyword"));
                    out.push_str(&buffer);
                    out.push_str(syntax::TerminalColors::reset());
                } else {
                    out.push_str(&buffer);
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
        fn validate(
            &self,
            _ctx: &mut rustyline::validate::ValidationContext,
        ) -> rustyline::Result<rustyline::validate::ValidationResult> {
            Ok(rustyline::validate::ValidationResult::Valid(None))
        }
    }

    impl Helper for MageCompleter {}

    pub fn run_repl(shell_override: Option<&str>) -> Result<(), String> {
        println!("🧙 Welcome to Mage REPL. Type 'exit' to quit.");

        // Check if tree-sitter is available and inform the user
        if syntax::is_tree_sitter_available() {
            println!("🎨 Tree-sitter syntax highlighting enabled");
        } else {
            println!("📝 Using basic syntax highlighting");
        }

        if let Some(shell) = shell_override {
            println!("🪄 Using shell: {}", shell);
        }

        let mut scope: HashMap<String, ExprValue> = HashMap::new();
        let mut functions: HashMap<String, FunctionDef> = HashMap::new();

        // Setup rustyline with our MageCompleter
        let config = Config::builder().auto_add_history(true).build();
        let completer = MageCompleter::new();
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
                            Ok(pairs) => {
                                interpret(pairs, shell_override, &mut scope, &mut functions)
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            }
        }

        Ok(())
    }
}
