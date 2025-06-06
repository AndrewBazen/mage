use crate::Rule;
use pest::iterators::Pairs;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct FunctionDef<'i> {
    params: Vec<String>,
    body: Vec<pest::iterators::Pair<'i, Rule>>,
}

#[derive(Debug, Clone)]
enum ExprValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl std::fmt::Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExprValue::String(s) => write!(f, "{}", s),
            ExprValue::Number(n) => write!(f, "{}", n),
            ExprValue::Boolean(b) => write!(f, "{}", b),
        }
    }
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
                        Rule::scry_chain => handle_scry_chain(stmt, scope, functions),
                        Rule::channel_block => handle_channel_block(stmt, scope, functions),
                        Rule::chant_block => handle_chant_block(stmt, scope),
                        Rule::recite_block => handle_recite_block(stmt, scope),
                        Rule::loop_block => handle_loop_block(stmt, scope, functions),
                        Rule::enchant => handle_enchant(stmt, functions),
                        Rule::cast => handle_cast(stmt, scope, functions),
                        _ => unreachable!("Unknown statement: {:?}", stmt),
                    }
                }
            }
        }
    }
}

fn match_incantation<'i>(stmt: pest::iterators::Pair<'i, Rule>, scope: &mut HashMap<String, String>, functions: &mut HashMap<String, FunctionDef<'i>>) {
    match stmt.as_rule() {
        Rule::incantation => {
            let mut inner = stmt.into_inner();
            let stmt = inner.next().unwrap();
            match_incantation(stmt, scope, functions);
        }
        Rule::statement => {
            let mut inner = stmt.into_inner();
            let stmt = inner.next().unwrap();
            match_incantation(stmt, scope, functions);
        }
        Rule::conjure | Rule::conjure_stmt => handle_conjure(stmt, scope),
        Rule::incant | Rule::incant_stmt => handle_incant(stmt, scope),
        Rule::curse | Rule::curse_stmt => handle_curse(stmt),
        Rule::evoke | Rule::evoke_stmt => handle_evoke(stmt, scope, None),
        Rule::scry_chain => handle_scry_chain(stmt, scope, functions),
        Rule::channel_block => handle_channel_block(stmt, scope, functions),
        Rule::chant_block => handle_chant_block(stmt, scope),
        Rule::recite_block => handle_recite_block(stmt, scope),
        Rule::loop_block => handle_loop_block(stmt, scope, functions),
        Rule::enchant => handle_enchant(stmt, functions),
        Rule::cast => handle_cast(stmt, scope, functions),
        _ => {} // Skip unhandled types like comments or strings
    }
}

fn handle_conjure(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let expression_pair = inner.next().unwrap();
    
    let value = evaluate_expression(expression_pair, scope).to_string();
    scope.insert(ident, value);
    // Note: semicolon is automatically handled by the grammar, no need to process it here
}

fn handle_incant(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let expression_pair = pair.into_inner().next().unwrap();
    let result = evaluate_expression(expression_pair, scope);
    
    let output = match result {
        ExprValue::String(s) => interpolate(&s, scope),
        ExprValue::Number(n) => n.to_string(),
        ExprValue::Boolean(b) => b.to_string(),
    };
    
    println!("{}", output);
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

fn handle_evoke(
    pair: pest::iterators::Pair<Rule>,
    scope: &mut HashMap<String, String>,
    shell_override: Option<&str>,
) {
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

fn handle_scry_chain<'i>(pair: pest::iterators::Pair<'i, Rule>, scope: &mut HashMap<String, String>, functions: &mut HashMap<String, FunctionDef<'i>>) {
    let mut inner = pair.into_inner();
    let scry_cond = inner.next().unwrap();
    let scry_block = inner.next().unwrap();
    
    let statements: Vec<_> = scry_block.into_inner().collect();
    
    // Check if scry condition is true
    if eval_condition(scry_cond, scope) {
        for stmt in statements {
            match_incantation(stmt, scope, functions);
        }
        return; // Exit early if scry block executed
    }
    
    // Check each morph block
    while let Some(morph) = inner.next() {
        if morph.as_rule() == Rule::morph_block {
            let mut morph_inner = morph.into_inner();
            let morph_cond = morph_inner.next().unwrap();
            let morph_block = morph_inner.next().unwrap();
            
            if eval_condition(morph_cond, scope) {
                for stmt in morph_block.into_inner() {
                    match_incantation(stmt, scope, functions);
                }
                return; // Exit early if morph block executed
            }
        } else if morph.as_rule() == Rule::lest_block {
            // This is the lest block, execute it since no conditions matched
            for stmt in morph.into_inner() {
                match_incantation(stmt, scope, functions);
            }
            return;
        }
    }
}

fn handle_loop_block<'i>(pair: pest::iterators::Pair<'i, Rule>, scope: &mut HashMap<String, String>, functions: &mut HashMap<String, FunctionDef<'i>>) {
    let block = pair.into_inner().next().unwrap();
    for _ in 0..3 {
        // Loop 3 times for demonstration
        for stmt in block.clone().into_inner() {
            match_incantation(stmt, scope, functions);
        }
    }
}

fn handle_enchant<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let params_pair = inner.next().unwrap();
    let mut params = Vec::new();
    if params_pair.as_rule() == Rule::param_list {
        params = params_pair
            .into_inner()
            .map(|p| p.as_str().to_string())
            .collect();
    }
    let body_pair = inner.next().unwrap();
    let body: Vec<_> = body_pair.into_inner().collect();
    let func = FunctionDef { params, body };
    functions.insert(name, func);
}

fn handle_cast<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    parent_scope: &mut HashMap<String, String>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let args_pair = inner.next();
    let mut args: Vec<String> = Vec::new();
    if let Some(args_pair) = args_pair {
        if args_pair.as_rule() == Rule::arg_list {
            args = args_pair
                .into_inner()
                .map(|a| a.as_str().trim_matches('"').to_string())
                .collect();
        }
    }
    if let Some(func) = functions.get(name) {
        let mut scope = parent_scope.clone();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            scope.insert(param.clone(), arg.clone());
        }
        for stmt in func.body.clone() {
            match_incantation(stmt, &mut scope, functions);
        }
    } else {
        eprintln!("❌ Unknown function: {}", name);
    }
}

fn handle_channel_block<'i>(pair: pest::iterators::Pair<'i, Rule>, scope: &mut HashMap<String, String>, functions: &mut HashMap<String, FunctionDef<'i>>) {
    let mut inner = pair.into_inner();
    let cond = inner.next().unwrap();
    let block = inner.next().unwrap();
    
    // Collect all statements once, outside the loop
    let statements: Vec<_> = block.into_inner().collect();
    
    let mut iteration_count = 0;
    while eval_condition(cond.clone(), scope) {
        iteration_count += 1;
        
        // Safety break to prevent infinite loops due to parsing bug
        if iteration_count > 10 {
            eprintln!("❌ Channel loop exceeded 10 iterations, breaking to prevent infinite loop");
            break;
        }
        
        // Execute the collected statements
        for stmt in &statements {
            match_incantation(stmt.clone(), scope, functions);
        }
    }
}

fn handle_chant_block(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let value = inner.next().unwrap().as_str().trim_matches('"').to_string();
    scope.insert(ident.clone(), value);
}

fn handle_recite_block(pair: pest::iterators::Pair<Rule>, scope: &mut HashMap<String, String>) {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let value = inner.next().unwrap().as_str().trim_matches('"').to_string();
    scope.insert(ident.clone(), value);
}




fn evaluate_expression(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>) -> ExprValue {
    match pair.as_rule() {
        Rule::expression => {
            let mut inner = pair.into_inner();
            let mut result = evaluate_term(inner.next().unwrap(), scope);
            
            while let Some(op) = inner.next() {
                let term = inner.next().unwrap();
                let term_val = evaluate_term(term, scope);
                result = apply_add_op(result, op.as_str(), term_val);
            }
            result
        }
        _ => {
            eprintln!("❌ Expected expression, got: {:?}", pair.as_rule());
            ExprValue::Number(0.0)
        }
    }
}

fn evaluate_term(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>) -> ExprValue {
    let mut inner = pair.into_inner();
    let mut result = evaluate_factor(inner.next().unwrap(), scope);
    
    while let Some(op) = inner.next() {
        let factor = inner.next().unwrap();
        let factor_val = evaluate_factor(factor, scope);
        result = apply_mult_op(result, op.as_str(), factor_val);
    }
    result
}

fn evaluate_factor(pair: pest::iterators::Pair<Rule>, scope: &HashMap<String, String>) -> ExprValue {
    match pair.as_rule() {
        Rule::factor => {
            // Factor rule contains the actual value or expression
            let inner = pair.into_inner().next().unwrap();
            evaluate_factor(inner, scope)
        }
        Rule::value => {
            let inner_value = pair.into_inner().next().unwrap();
            match inner_value.as_rule() {
                Rule::string => ExprValue::String(inner_value.as_str().trim_matches('"').to_string()),
                Rule::number => ExprValue::Number(inner_value.as_str().parse().unwrap_or(0.0)),
                Rule::boolean => ExprValue::Boolean(inner_value.as_str() == "true"),
                Rule::IDENT => {
                    let var_name = inner_value.as_str();
                    if let Some(val) = scope.get(var_name) {
                        // Try to parse as number first, then boolean, then string
                        if let Ok(num) = val.parse::<f64>() {
                            ExprValue::Number(num)
                        } else if val == "true" || val == "false" {
                            ExprValue::Boolean(val == "true")
                        } else {
                            ExprValue::String(val.clone())
                        }
                    } else {
                        ExprValue::String(format!("${{{}}}", var_name))
                    }
                }
                _ => ExprValue::Number(0.0)
            }
        }
        Rule::expression => evaluate_expression(pair, scope),
        _ => {
            eprintln!("❌ Unexpected factor: {:?}", pair.as_rule());
            ExprValue::Number(0.0)
        }
    }
}

fn apply_add_op(left: ExprValue, op: &str, right: ExprValue) -> ExprValue {
    match (&left, &right) {
        (ExprValue::Number(l), ExprValue::Number(r)) => {
            match op {
                "+" => ExprValue::Number(l + r),
                "-" => ExprValue::Number(l - r),
                _ => ExprValue::Number(0.0)
            }
        }
        (ExprValue::String(l), ExprValue::String(r)) if op == "+" => {
            ExprValue::String(format!("{}{}", l, r))
        }
        _ => {
            eprintln!("❌ Invalid operation: {} {} {}", left, op, right);
            ExprValue::Number(0.0)
        }
    }
}

fn apply_mult_op(left: ExprValue, op: &str, right: ExprValue) -> ExprValue {
    match (&left, &right) {
        (ExprValue::Number(l), ExprValue::Number(r)) => {
            match op {
                "*" => ExprValue::Number(l * r),
                "/" => ExprValue::Number(if *r != 0.0 { l / r } else { 0.0 }),
                "%" => ExprValue::Number(if *r != 0.0 { l % r } else { 0.0 }),
                _ => ExprValue::Number(0.0)
            }
        }
        _ => {
            eprintln!("❌ Invalid operation: {} {} {}", left, op, right);
            ExprValue::Number(0.0)
        }
    }
}

fn eval_condition<'i>(pair: pest::iterators::Pair<'i, Rule>, scope: &mut HashMap<String, String>) -> bool {
    let mut inner = pair.into_inner();
    let left_expr = inner.next().unwrap();
    let cmp = inner.next().unwrap().as_str();
    let right_expr = inner.next().unwrap();
    
    let left_val = evaluate_expression(left_expr, scope);
    let right_val = evaluate_expression(right_expr, scope);
    
    match cmp {
        "==" => compare_values(&left_val, &right_val) == std::cmp::Ordering::Equal,
        "!=" => compare_values(&left_val, &right_val) != std::cmp::Ordering::Equal,
        ">" => compare_values(&left_val, &right_val) == std::cmp::Ordering::Greater,
        "<" => compare_values(&left_val, &right_val) == std::cmp::Ordering::Less,
        ">=" => {
            let ord = compare_values(&left_val, &right_val);
            ord == std::cmp::Ordering::Greater || ord == std::cmp::Ordering::Equal
        }
        "<=" => {
            let ord = compare_values(&left_val, &right_val);
            ord == std::cmp::Ordering::Less || ord == std::cmp::Ordering::Equal
        }
        _ => false,
    }
}

fn compare_values(left: &ExprValue, right: &ExprValue) -> std::cmp::Ordering {
    match (left, right) {
        (ExprValue::Number(l), ExprValue::Number(r)) => l.partial_cmp(r).unwrap_or(std::cmp::Ordering::Equal),
        (ExprValue::String(l), ExprValue::String(r)) => l.cmp(r),
        (ExprValue::Boolean(l), ExprValue::Boolean(r)) => l.cmp(r),
        // Mixed types: convert to strings for comparison
        _ => left.to_string().cmp(&right.to_string()),
    }
}

fn interpolate<'i>(text: &str, scope: &mut HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Handle escape sequences
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    '$' => {
                        result.push('$');
                        chars.next();
                    }
                    '{' => {
                        result.push('{');
                        chars.next();
                    }
                    '}' => {
                        result.push('}');
                        chars.next();
                    }
                    _ => {
                        result.push(next_ch);
                        chars.next();
                    }
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
