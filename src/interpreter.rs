use pest::iterators::Pairs;
use crate::Rule;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct FunctionDef<'i> {
    params: Vec<String>,
    body: Vec<pest::iterators::Pair<'i, Rule>>,
}

pub fn interpret<'i>(
    pairs: Pairs<'i, Rule>,
    shell_override: Option<&str>,
    scope: &mut HashMap<String, String>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
) {
    for pair in pairs {
        if pair.as_rule() == Rule::program {
            for incantation in pair.into_inner() {
                if incantation.as_rule() == Rule::incantation {
                    let mut inner = incantation.into_inner();
                    let stmt = inner.next().unwrap();

                    match stmt.as_rule() {
                        Rule::conjure => handle_conjure(stmt, scope),
                        Rule::incant => handle_incant(stmt, scope),
                        Rule::curse => handle_curse(stmt),
                        Rule::evoke => handle_evoke(stmt, scope, shell_override),
                        Rule::if_block => handle_if_block(stmt, scope),
                        Rule::loop_block => handle_loop_block(stmt, scope),
                        Rule::enchant => handle_enchant(stmt, functions),
                        Rule::cast => handle_cast(stmt, scope, functions),
                        _ => unreachable!("Unknown statement: {:?}", stmt),
                    }
                }
            }
        }
    }
}

fn handle_conjure(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let value = inner.next().unwrap().as_str().trim_matches('"').to_string();
    scope.insert(ident.clone(), value);
}

fn handle_incant(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>) {
    let raw = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    let interpolated = interpolate(raw, scope);
    println!("{}", interpolated);
    io::stdout().flush().unwrap();
}

fn handle_curse(pair: pest::iterators::Pair<Rule>) {
    let message = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    eprintln!("❌ CURSE: {}", message);
    std::process::exit(1);
}

#[cfg(target_family = "windows")]
fn shell_command(command: &str, _shell_override: Option<&str>) -> std::process::Command {
    let mut cmd = std::process::Command::new("cmd");
    cmd.arg("/C").arg(command);
    cmd
}

#[cfg(not(target_family = "windows"))]
fn shell_command(command: &str, shell_override: Option<&str>) -> std::process::Command {
    if let Some(shell) = shell_override {
        let mut cmd = std::process::Command::new(shell);
        cmd.arg("-c").arg(command);
        return cmd;
    }
    if let Ok(shell) = env::var("MAGE_SHELL") {
        let mut cmd = std::process::Command::new(shell);
        cmd.arg("-c").arg(command);
        return cmd;
    }
    if let Ok(shell) = env::var("SHELL") {
        let mut cmd = std::process::Command::new(shell);
        cmd.arg("-c").arg(command);
        return cmd;
    }
    for shell in &["bash", "zsh", "fish", "sh"] {
        if which::which(shell).is_ok() {
            let mut cmd = std::process::Command::new(shell);
            cmd.arg("-c").arg(command);
            return cmd;
        }
    }
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c").arg(command);
    cmd
}

fn handle_evoke(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>, shell_override: Option<&str>) {
    let raw = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    let command = interpolate(raw, scope);
    let output = shell_command(&command, shell_override).output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
            if !output.status.success() {
                std::process::exit(output.status.code().unwrap_or(1));
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to evoke command: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_if_block(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let mut inner = pair.into_inner();
    let cond = inner.next().unwrap();
    let block = inner.next().unwrap();
    if eval_condition(cond, scope) {
        for stmt in block.into_inner() {
            if stmt.as_rule() == Rule::incantation {
                let mut inner = stmt.into_inner();
                let stmt = inner.next().unwrap();
                match stmt.as_rule() {
                    Rule::conjure => handle_conjure(stmt, scope),
                    Rule::incant => handle_incant(stmt, scope),
                    Rule::curse => handle_curse(stmt),
                    Rule::evoke => handle_evoke(stmt, scope, None),
                    Rule::if_block => handle_if_block(stmt, scope),
                    Rule::loop_block => handle_loop_block(stmt, scope),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn handle_loop_block(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let block = pair.into_inner().next().unwrap();
    for _ in 0..3 { // Loop 3 times for demonstration
        for stmt in block.clone().into_inner() {
            if stmt.as_rule() == Rule::incantation {
                let mut inner = stmt.into_inner();
                let stmt = inner.next().unwrap();
                match stmt.as_rule() {
                    Rule::conjure => handle_conjure(stmt, scope),
                    Rule::incant => handle_incant(stmt, scope),
                    Rule::curse => handle_curse(stmt),
                    Rule::evoke => handle_evoke(stmt, scope, None),
                    Rule::if_block => handle_if_block(stmt, scope),
                    Rule::loop_block => handle_loop_block(stmt, scope),
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn handle_enchant<'i>(pair: pest::iterators::Pair<'i, Rule>, functions: &mut HashMap<String, FunctionDef<'i>>) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let params_pair = inner.next().unwrap();
    let mut params = Vec::new();
    if params_pair.as_rule() == Rule::param_list {
        params = params_pair.into_inner().map(|p| p.as_str().to_string()).collect();
    }
    let body_pair = inner.next().unwrap();
    let body: Vec<_> = body_pair.into_inner().collect();
    let func = FunctionDef { params, body };
    functions.insert(name, func);
}

fn handle_cast<'i>(pair: pest::iterators::Pair<'i, Rule>, parent_scope: &mut HashMap<String, String>, functions: &HashMap<String, FunctionDef<'i>>) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let args_pair = inner.next();
    let mut args: Vec<String> = Vec::new();
    if let Some(args_pair) = args_pair {
        if args_pair.as_rule() == Rule::arg_list {
            args = args_pair.into_inner().map(|a| a.as_str().trim_matches('"').to_string()).collect();
        }
    }
    if let Some(func) = functions.get(name) {
        let mut scope = parent_scope.clone();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            scope.insert(param.clone(), arg.clone());
        }
        for stmt in &func.body {
            if stmt.as_rule() == Rule::incantation {
                let mut inner = stmt.clone().into_inner();
                let stmt = inner.next().unwrap();
                match stmt.as_rule() {
                    Rule::conjure => handle_conjure(stmt, &mut scope),
                    Rule::incant => handle_incant(stmt, &scope),
                    Rule::curse => handle_curse(stmt),
                    Rule::evoke => handle_evoke(stmt, &scope, None),
                    Rule::if_block => handle_if_block(stmt, &mut scope),
                    Rule::loop_block => handle_loop_block(stmt, &mut scope),
                    Rule::enchant => {/* ignore nested enchant for now */},
                    Rule::cast => handle_cast(stmt, &mut scope, functions),
                    _ => unreachable!(),
                }
            }
        }
    } else {
        eprintln!("❌ Unknown function: {}", name);
    }
}

fn eval_condition(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>) -> bool {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str();
    let cmp = inner.next().unwrap().as_str();
    let val = inner.next().unwrap().as_str().trim_matches('"');
    let left = scope.get(ident).map(|s| s.as_str()).unwrap_or("");
    match cmp {
        "==" => left == val,
        "!=" => left != val,
        _ => false,
    }
}

fn interpolate(text: &str, scope: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Handle escape sequences
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    '\\' => { result.push('\\'); chars.next(); }
                    '$'  => { result.push('$'); chars.next(); }
                    '{'  => { result.push('{'); chars.next(); }
                    '}'  => { result.push('}'); chars.next(); }
                    _    => { result.push(next_ch); chars.next(); }
                }
            }
        } else if ch == '$' {
            if let Some(&'{') = chars.peek() {
                chars.next(); // consume '{'
                let mut var = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '}' {
                        chars.next(); // consume '}'
                        break;
                    } else {
                        var.push(c);
                        chars.next();
                    }
                }
                if let Some(value) = scope.get(&var) {
                    result.push_str(value);
                } else {
                    result.push_str(&format!("${{{}}}", var));
                }
            } else {
                // $var style
                let mut var = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        var.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Some(value) = scope.get(&var) {
                    result.push_str(value);
                } else {
                    result.push_str(&format!("${}", var));
                }
            }
        } else {
            result.push(ch);
        }
    }

    result
}
