use crate::{Rule, builtins};
use crate::output::{OutputCollector, InterpreterError};
use pest::iterators::Pairs;
use std::collections::HashMap;

#[cfg(not(target_family = "windows"))]
use std::env;

#[derive(Clone)]
pub struct FunctionDef<'i> {
    params: Vec<String>,
    body: Vec<pest::iterators::Pair<'i, Rule>>,
}

#[derive(Debug, Clone)]
pub enum FunctionResult {
    None,
    Value(ExprValue),
    Return(ExprValue),
}

#[derive(Debug, Clone)]
pub enum ExprValue {
    String(String),
    Number(f64),
    Boolean(bool),
    List(Vec<ExprValue>),
    Map(HashMap<String, ExprValue>),
}

impl std::fmt::Display for ExprValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ExprValue::String(s) => write!(f, "{}", s),
            ExprValue::Number(n) => write!(f, "{}", n),
            ExprValue::Boolean(b) => write!(f, "{}", b),
            ExprValue::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            ExprValue::Map(m) => write!(
                f,
                "{{{}}}",
                m.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

impl ExprValue {
    pub fn to_display_string(&self) -> String {
        match self {
            ExprValue::String(s) => s.clone(),
            other => other.to_string(),
        }
    }
}

pub fn interpret<'i>(
    pairs: Pairs<'i, Rule>,
    shell_override: Option<&str>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    for pair in pairs {
        if pair.as_rule() == Rule::program {
            for incantation in pair.into_inner() {
                match_incantation_with_shell(incantation, scope, functions, shell_override, output)?;
            }
        }
    }
    Ok(())
}

/// Top-level incantation handler that supports shell override
fn match_incantation_with_shell<'i>(
    stmt: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    shell_override: Option<&str>,
    output: &mut OutputCollector,
) -> Result<Option<ExprValue>, InterpreterError> {
    match stmt.as_rule() {
        // Unwrap the incantation wrapper to get the actual statement
        Rule::incantation => {
            let inner = stmt.into_inner().next().unwrap();
            match_incantation_with_shell(inner, scope, functions, shell_override, output)
        }
        Rule::evoke => {
            handle_evoke(stmt, scope, shell_override, output)?;
            Ok(None)
        }
        _ => match_incantation(stmt, scope, functions, output),
    }
}

fn match_incantation<'i>(
    stmt: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<Option<ExprValue>, InterpreterError> {
    match stmt.as_rule() {
        Rule::conjure => {
            handle_conjure(stmt, scope, functions, output);
            Ok(None)
        }
        Rule::incant => {
            handle_incant(stmt, scope, functions, output);
            Ok(None)
        }
        Rule::curse => {
            handle_curse(stmt, output)?;
            Ok(None)
        }
        Rule::evoke => {
            handle_evoke(stmt, scope, None, output)?;
            Ok(None)
        }
        Rule::scry_chain => {
            handle_scry_chain(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::channel_block => {
            handle_channel_block(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::chant_block => {
            handle_chant_block(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::recite_block => {
            handle_recite_block(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::loop_block => {
            handle_loop_block(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::enchant => {
            handle_enchant(stmt, functions);
            Ok(None)
        }
        Rule::cast => {
            handle_cast(stmt, scope, functions, output)?;
            Ok(None)
        }
        Rule::bestow => Ok(Some(handle_bestow(stmt, scope, functions, output))),
        Rule::yield_stmt => Ok(Some(handle_yield(stmt, scope, functions, output))),
        _ => Ok(None), // Skip unhandled types like comments or EOI
    }
}

// ─── Variable Declaration ────────────────────────────────────────────

fn handle_conjure(
    pair: pest::iterators::Pair<Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) {
    let mut inner = pair.into_inner();
    let ident = inner.next().unwrap().as_str().to_string();
    let expression_pair = inner.next().unwrap();

    let value = evaluate_expression(expression_pair, scope, functions, output);
    scope.insert(ident, value);
}

// ─── Output ──────────────────────────────────────────────────────────

fn handle_incant(
    pair: pest::iterators::Pair<Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) {
    let expression_pair = pair.into_inner().next().unwrap();
    let result = evaluate_expression(expression_pair, scope, functions, output);

    let text = match result {
        ExprValue::String(s) => interpolate(&s, scope),
        other => other.to_display_string(),
    };

    output.println(&text);
}

// ─── Error / Exit ────────────────────────────────────────────────────

fn handle_curse(
    pair: pest::iterators::Pair<Rule>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let message = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    output.eprintln(&format!("CURSE: {}", message));
    Err(InterpreterError::Curse(message.to_string()))
}

// ─── Shell Commands ──────────────────────────────────────────────────

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
    scope: &mut HashMap<String, ExprValue>,
    shell_override: Option<&str>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let raw = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    let command = interpolate(raw, scope);
    let cmd_output = shell_command(&command, shell_override).output();

    match cmd_output {
        Ok(cmd_output) => {
            if !cmd_output.stdout.is_empty() {
                output.print(&String::from_utf8_lossy(&cmd_output.stdout));
            }
            if !cmd_output.stderr.is_empty() {
                output.eprint(&String::from_utf8_lossy(&cmd_output.stderr));
            }
            if !cmd_output.status.success() {
                return Err(InterpreterError::CommandFailed(
                    cmd_output.status.code().unwrap_or(1),
                ));
            }
            Ok(())
        }
        Err(e) => {
            output.eprintln(&format!("Failed to evoke command: {}", e));
            Err(InterpreterError::CommandError(e.to_string()))
        }
    }
}

fn handle_scry_chain<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let mut inner = pair.into_inner();
    let scry_cond = inner.next().unwrap();
    let scry_block = inner.next().unwrap();

    let statements: Vec<_> = scry_block.into_inner().collect();

    // Check if scry condition is true
    if eval_condition(scry_cond, scope, functions, output) {
        for stmt in statements {
            match_incantation(stmt, scope, functions, output)?;
        }
        return Ok(());
    }

    // Check each morph block
    for morph in inner {
        if morph.as_rule() == Rule::morph_block {
            let mut morph_inner = morph.into_inner();
            let morph_cond = morph_inner.next().unwrap();
            let morph_block = morph_inner.next().unwrap();

            if eval_condition(morph_cond, scope, functions, output) {
                for stmt in morph_block.into_inner() {
                    match_incantation(stmt, scope, functions, output)?;
                }
                return Ok(());
            }
        } else if morph.as_rule() == Rule::lest_block {
            for stmt in morph.into_inner() {
                match_incantation(stmt, scope, functions, output)?;
            }
            return Ok(());
        }
    }
    Ok(())
}

// ─── Loops ───────────────────────────────────────────────────────────

fn handle_loop_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let block = pair.into_inner().next().unwrap();
    for _ in 0..3 {
        for stmt in block.clone().into_inner() {
            match_incantation(stmt, scope, functions, output)?;
        }
    }
    Ok(())
}

fn handle_channel_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let mut inner = pair.into_inner();
    let cond = inner.next().unwrap();
    let block = inner.next().unwrap();

    let statements: Vec<_> = block.into_inner().collect();

    let mut iteration_count = 0;
    while eval_condition(cond.clone(), scope, functions, output) {
        iteration_count += 1;

        if iteration_count > 10 {
            output.eprintln("Channel loop exceeded 10 iterations, breaking to prevent infinite loop");
            break;
        }

        for stmt in &statements {
            match_incantation(stmt.clone(), scope, functions, output)?;
        }
    }
    Ok(())
}

fn handle_chant_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let mut inner = pair.into_inner();
    let loop_var = inner.next().unwrap().as_str().to_string();
    let start_expr = inner.next().unwrap();
    let end_expr = inner.next().unwrap();

    let mut step_expr = None;
    let mut block = None;

    for part in inner {
        if part.as_rule() == Rule::expression {
            step_expr = Some(part);
        } else if part.as_rule() == Rule::block {
            block = Some(part);
            break;
        }
    }

    let block = block.expect("Expected block in chant statement");
    let statements: Vec<_> = block.into_inner().collect();

    let start_val = evaluate_expression(start_expr, scope, functions, output);
    let end_val = evaluate_expression(end_expr, scope, functions, output);
    let step_val = if let Some(step) = step_expr {
        evaluate_expression(step, scope, functions, output)
    } else {
        ExprValue::Number(1.0)
    };

    let start_num = expr_to_i32(&start_val, "Start", output);
    let end_num = expr_to_i32(&end_val, "End", output);
    let step_num = expr_to_i32(&step_val, "Step", output);

    let (start_num, end_num, step_num) = match (start_num, end_num, step_num) {
        (Some(s), Some(e), Some(st)) => (s, e, st),
        _ => return Ok(()),
    };

    if step_num == 0 {
        output.eprintln("Step cannot be zero");
        return Ok(());
    }

    if step_num > 0 {
        let mut current = start_num;
        while current < end_num {
            scope.insert(loop_var.clone(), ExprValue::Number(current as f64));
            for stmt in &statements {
                match_incantation(stmt.clone(), scope, functions, output)?;
            }
            current += step_num;
        }
    } else {
        let mut current = start_num;
        while current > end_num {
            scope.insert(loop_var.clone(), ExprValue::Number(current as f64));
            for stmt in &statements {
                match_incantation(stmt.clone(), scope, functions, output)?;
            }
            current += step_num;
        }
    }
    Ok(())
}

fn handle_recite_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let mut inner = pair.into_inner();
    let loop_var = inner.next().unwrap().as_str().to_string();
    let list_expr = inner.next().unwrap();
    let block = inner.next().unwrap();

    let list_val = evaluate_factor(list_expr, scope, functions, output);

    match list_val {
        ExprValue::String(s) => {
            if s.trim().is_empty() {
                return Ok(());
            }
            let items: Vec<&str> = s
                .split(',')
                .map(|s| s.trim())
                .filter(|item| !item.is_empty())
                .collect();
            for item in items {
                scope.insert(loop_var.clone(), ExprValue::String(item.to_string()));
                for stmt in block.clone().into_inner() {
                    match_incantation(stmt, scope, functions, output)?;
                }
            }
        }
        ExprValue::Number(n) => {
            for i in 0..(n as i32) {
                scope.insert(loop_var.clone(), ExprValue::Number(i as f64));
                for stmt in block.clone().into_inner() {
                    match_incantation(stmt, scope, functions, output)?;
                }
            }
        }
        ExprValue::Boolean(_b) => {
            scope.insert(loop_var.clone(), ExprValue::Number(0.0));
            for stmt in block.clone().into_inner() {
                match_incantation(stmt, scope, functions, output)?;
            }
        }
        ExprValue::List(l) => {
            for item in l {
                scope.insert(loop_var.clone(), item);
                for stmt in block.clone().into_inner() {
                    match_incantation(stmt, scope, functions, output)?;
                }
            }
        }
        ExprValue::Map(m) => {
            for (key, _value) in m {
                scope.insert(loop_var.clone(), ExprValue::String(key));
                for stmt in block.clone().into_inner() {
                    match_incantation(stmt, scope, functions, output)?;
                }
            }
        }
    }
    Ok(())
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
    parent_scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Result<(), InterpreterError> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let args_pair = inner.next();

    let args = resolve_args(args_pair, parent_scope, functions, output);

    if builtins::is_builtin(name) {
        let string_args: Vec<String> = args.iter().map(|a| a.to_display_string()).collect();
        match builtins::call_builtin(name, string_args, output) {
            Ok(result) => {
                match result {
                    builtins::BuiltinValue::None => {}
                    builtins::BuiltinValue::Boolean(true) => {}
                    _ => output.println(&format!("{}", result)),
                }
            }
            Err(e) => output.eprintln(&format!("Error calling {}: {}", name, e)),
        }
    } else if let Some(func) = functions.get(name) {
        let mut scope = parent_scope.clone();
        for (param, arg) in func.params.iter().zip(args.into_iter()) {
            scope.insert(param.clone(), arg);
        }
        for stmt in func.body.clone() {
            if let Some(_val) = match_incantation(stmt, &mut scope, functions, output)? {
                return Ok(());
            }
        }
    } else {
        output.eprintln(&format!("Unknown function: {}", name));
    }
    Ok(())
}

/// Resolve an argument list from parsed pairs into typed ExprValues
fn resolve_args(
    args_pair: Option<pest::iterators::Pair<Rule>>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> Vec<ExprValue> {
    let Some(args_pair) = args_pair else {
        return Vec::new();
    };
    if args_pair.as_rule() != Rule::arg_list {
        return Vec::new();
    }

    args_pair
        .into_inner()
        .map(|a| resolve_single_arg(a, scope, functions, output))
        .collect()
}

/// Handle the bestow statement to return a value to the parent scope
fn handle_bestow(
    pair: pest::iterators::Pair<Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    let expression_pair = pair.into_inner().next().unwrap();
    evaluate_expression(expression_pair, scope, functions, output)
}

/// Handle the yield statement to return a value (alias for bestow)
fn handle_yield(
    pair: pest::iterators::Pair<Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    let expression_pair = pair.into_inner().next().unwrap();
    evaluate_expression(expression_pair, scope, functions, output)
}

/// Resolve a single argument value, evaluating it as a factor
fn resolve_single_arg(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    evaluate_factor(pair, scope, functions, output)
}

// ─── Expression Evaluation ───────────────────────────────────────────

fn evaluate_expression(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    match pair.as_rule() {
        Rule::expression => {
            let mut inner = pair.into_inner();
            let mut result = evaluate_term(inner.next().unwrap(), scope, functions, output);

            while let Some(op) = inner.next() {
                let term = inner.next().unwrap();
                let term_val = evaluate_term(term, scope, functions, output);
                result = apply_add_op(result, op.as_str(), term_val, output);
            }
            result
        }
        _ => {
            output.eprintln(&format!("Expected expression, got: {:?}", pair.as_rule()));
            ExprValue::Number(0.0)
        }
    }
}

fn evaluate_term(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    let mut inner = pair.into_inner();
    let mut result = evaluate_factor(inner.next().unwrap(), scope, functions, output);

    while let Some(op) = inner.next() {
        let factor = inner.next().unwrap();
        let factor_val = evaluate_factor(factor, scope, functions, output);
        result = apply_mult_op(result, op.as_str(), factor_val, output);
    }
    result
}

fn evaluate_builtin_function_call(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let args_pair = inner.next();

    let args = resolve_args(args_pair, scope, functions, output);

    if builtins::is_builtin(name) {
        let string_args: Vec<String> = args.iter().map(|a| a.to_display_string()).collect();
        match builtins::call_builtin(name, string_args, output) {
            Ok(result) => builtin_to_expr(result),
            Err(e) => {
                output.eprintln(&format!("Error calling {}: {}", name, e));
                ExprValue::String("".to_string())
            }
        }
    } else if let Some(func) = functions.get(name) {
        let mut func_scope = scope.clone();
        for (param, arg) in func.params.iter().zip(args.iter()) {
            func_scope.insert(param.clone(), arg.clone());
        }

        for stmt in func.body.clone() {
            match match_incantation(stmt, &mut func_scope, functions, output) {
                Ok(Some(val)) => return val,
                Ok(None) => {}
                Err(_) => return ExprValue::String("".to_string()),
            }
        }

        ExprValue::String("".to_string())
    } else {
        output.eprintln(&format!("Unknown function: {}", name));
        ExprValue::String("".to_string())
    }
}

fn evaluate_factor(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    match pair.as_rule() {
        Rule::factor => {
            let inner = pair.into_inner().next().unwrap();
            evaluate_factor(inner, scope, functions, output)
        }
        Rule::value => {
            let inner_value = pair.into_inner().next().unwrap();
            match inner_value.as_rule() {
                Rule::string => ExprValue::String(process_escape_sequences(
                    inner_value.as_str().trim_matches('"'),
                )),
                Rule::number => ExprValue::Number(inner_value.as_str().parse().unwrap_or(0.0)),
                Rule::boolean => ExprValue::Boolean(inner_value.as_str() == "true"),
                Rule::list => ExprValue::List(
                    inner_value
                        .into_inner()
                        .map(|v| evaluate_expression(v, scope, functions, output))
                        .collect(),
                ),
                Rule::map => ExprValue::Map(
                    inner_value
                        .into_inner()
                        .map(|v| {
                            let mut parts = v.into_inner();
                            let key = parts.next().unwrap().as_str();
                            let value = parts.next().unwrap();
                            (
                                key.to_string(),
                                evaluate_expression(value, scope, functions, output),
                            )
                        })
                        .collect(),
                ),
                Rule::IDENT => {
                    let var_name = inner_value.as_str();
                    if let Some(val) = scope.get(var_name) {
                        val.clone()
                    } else {
                        ExprValue::String(format!("${{{}}}", var_name))
                    }
                }
                Rule::cast => evaluate_builtin_function_call(inner_value, scope, functions, output),
                _ => ExprValue::Number(0.0),
            }
        }
        Rule::expression => evaluate_expression(pair, scope, functions, output),
        _ => {
            output.eprintln(&format!("Unexpected factor: {:?}", pair.as_rule()));
            ExprValue::Number(0.0)
        }
    }
}

// ─── Operators ───────────────────────────────────────────────────────

fn apply_add_op(left: ExprValue, op: &str, right: ExprValue, output: &mut OutputCollector) -> ExprValue {
    match (&left, &right) {
        (ExprValue::Number(l), ExprValue::Number(r)) => match op {
            "+" => ExprValue::Number(l + r),
            "-" => ExprValue::Number(l - r),
            _ => ExprValue::Number(0.0),
        },
        (ExprValue::String(l), ExprValue::String(r)) if op == "+" => {
            ExprValue::String(format!("{}{}", l, r))
        }
        (ExprValue::String(l), right_val) if op == "+" => {
            ExprValue::String(format!("{}{}", l, right_val))
        }
        (left_val, ExprValue::String(r)) if op == "+" => {
            ExprValue::String(format!("{}{}", left_val, r))
        }
        (ExprValue::List(l), ExprValue::List(r)) if op == "+" => {
            ExprValue::List(l.iter().chain(r.iter()).cloned().collect())
        }
        (ExprValue::Map(l), ExprValue::Map(r)) if op == "+" => ExprValue::Map(
            l.iter()
                .chain(r.iter())
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        ),
        _ => {
            output.eprintln(&format!("Invalid operation: {} {} {}", left, op, right));
            ExprValue::Number(0.0)
        }
    }
}

fn apply_mult_op(left: ExprValue, op: &str, right: ExprValue, output: &mut OutputCollector) -> ExprValue {
    match (&left, &right) {
        (ExprValue::Number(l), ExprValue::Number(r)) => match op {
            "*" => ExprValue::Number(l * r),
            "/" => ExprValue::Number(if *r != 0.0 { l / r } else { 0.0 }),
            "%" => ExprValue::Number(if *r != 0.0 { l % r } else { 0.0 }),
            _ => ExprValue::Number(0.0),
        },
        _ => {
            output.eprintln(&format!("Invalid operation: {} {} {}", left, op, right));
            ExprValue::Number(0.0)
        }
    }
}

// ─── Conditions ──────────────────────────────────────────────────────

fn eval_condition(
    pair: pest::iterators::Pair<'_, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> bool {
    let mut inner = pair.into_inner();
    let left_expr = inner.next().unwrap();
    let cmp = inner.next().unwrap().as_str();
    let right_expr = inner.next().unwrap();

    let left_val = evaluate_expression(left_expr, scope, functions, output);
    let right_val = evaluate_expression(right_expr, scope, functions, output);

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
        (ExprValue::Number(l), ExprValue::Number(r)) => {
            l.partial_cmp(r).unwrap_or(std::cmp::Ordering::Equal)
        }
        (ExprValue::String(l), ExprValue::String(r)) => l.cmp(r),
        (ExprValue::Boolean(l), ExprValue::Boolean(r)) => l.cmp(r),
        (ExprValue::List(l), ExprValue::List(r)) => {
            let mut l_iter = l.iter();
            let mut r_iter = r.iter();
            while let (Some(l_val), Some(r_val)) = (l_iter.next(), r_iter.next()) {
                let ord = compare_values(l_val, r_val);
                if ord != std::cmp::Ordering::Equal {
                    return ord;
                }
            }
            l.len().cmp(&r.len())
        }
        (ExprValue::Map(l), ExprValue::Map(r)) => {
            let mut l_keys = l.keys().collect::<Vec<&String>>();
            let mut r_keys = r.keys().collect::<Vec<&String>>();
            l_keys.sort();
            r_keys.sort();
            l_keys.cmp(&r_keys)
        }
        _ => left.to_string().cmp(&right.to_string()),
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────

fn expr_to_i32(val: &ExprValue, label: &str, output: &mut OutputCollector) -> Option<i32> {
    match val {
        ExprValue::Number(n) => Some(*n as i32),
        ExprValue::String(s) => s.parse().ok().or_else(|| {
            output.eprintln(&format!("{} value must be a number, got string: {}", label, s));
            None
        }),
        ExprValue::List(l) => Some(l.len() as i32),
        ExprValue::Map(m) => Some(m.len() as i32),
        ExprValue::Boolean(_) => {
            output.eprintln(&format!("{} value must be a number", label));
            None
        }
    }
}

/// Convert a builtins::BuiltinValue to an ExprValue.
fn builtin_to_expr(val: builtins::BuiltinValue) -> ExprValue {
    match val {
        builtins::BuiltinValue::None => ExprValue::String("".to_string()),
        builtins::BuiltinValue::String(s) => ExprValue::String(s),
        builtins::BuiltinValue::Number(n) => ExprValue::Number(n),
        builtins::BuiltinValue::Boolean(b) => ExprValue::Boolean(b),
        builtins::BuiltinValue::Array(l) => {
            ExprValue::List(l.into_iter().map(ExprValue::String).collect())
        }
    }
}

fn process_escape_sequences(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                match next_ch {
                    '\\' => {
                        result.push('\\');
                        chars.next();
                    }
                    'n' => {
                        result.push('\n');
                        chars.next();
                    }
                    't' => {
                        result.push('\t');
                        chars.next();
                    }
                    'r' => {
                        result.push('\r');
                        chars.next();
                    }
                    '"' => {
                        result.push('"');
                        chars.next();
                    }
                    '\'' => {
                        result.push('\'');
                        chars.next();
                    }
                    '0' => {
                        result.push('\0');
                        chars.next();
                    }
                    _ => {
                        result.push('\\');
                        result.push(next_ch);
                        chars.next();
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn interpolate(text: &str, scope: &HashMap<String, ExprValue>) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
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
                chars.next();
                let mut var = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '}' {
                        chars.next();
                        break;
                    } else {
                        var.push(c);
                        chars.next();
                    }
                }
                if let Some(value) = scope.get(&var) {
                    result.push_str(&value.to_display_string());
                } else {
                    result.push_str(&format!("${{{}}}", var));
                }
            } else {
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
                    result.push_str(&value.to_display_string());
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
