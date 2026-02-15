use crate::output::{InterpreterError, OutputCollector};
use crate::{Rule, builtins};
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

#[derive(Debug, Clone)]
enum Signal {
    None,              // keep going
    Return(ExprValue), // bestow - exit function with value
    Break,             // dispel - exit loop
    Continue,          // portal - next iteration
    Error(String),     // curse/summon - error
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
                match match_incantation_with_shell(
                    incantation,
                    scope,
                    functions,
                    shell_override,
                    output,
                ) {
                    Signal::Error(msg) => {
                        return Err(InterpreterError::Curse(msg));
                    }
                    _ => {}
                }
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
) -> Signal {
    match stmt.as_rule() {
        Rule::incantation => {
            let inner = stmt.into_inner().next().unwrap();
            match_incantation_with_shell(inner, scope, functions, shell_override, output)
        }
        Rule::evoke => {
            handle_evoke(stmt, scope, shell_override, output);
            Signal::None
        }
        _ => match_incantation(stmt, scope, functions, output),
    }
}

fn match_incantation<'i>(
    stmt: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    match stmt.as_rule() {
        Rule::incantation => {
            let inner = stmt.into_inner().next().unwrap();
            match_incantation(inner, scope, functions, output)
        }
        Rule::conjure => {
            handle_conjure(stmt, scope, functions, output);
            Signal::None
        }
        Rule::incant => {
            handle_incant(stmt, scope, functions, output);
            Signal::None
        }
        Rule::curse => {
            handle_curse(stmt, output);
            Signal::None // Never reached - curse exits the program
        }
        Rule::summon => handle_summon(stmt, scope, functions, output),
        Rule::evoke => {
            handle_evoke(stmt, scope, None, output);
            Signal::None
        }
        Rule::scry_chain => handle_scry_chain(stmt, scope, functions, output),
        Rule::invoke_block => handle_invoke_block(stmt, scope, functions, output),
        Rule::channel_block => handle_channel_block(stmt, scope, functions, output),
        Rule::chant_block => handle_chant_block(stmt, scope, functions, output),
        Rule::recite_block => handle_recite_block(stmt, scope, functions, output),
        Rule::loop_block => handle_loop_block(stmt, scope, functions, output),
        Rule::enchant => {
            handle_enchant(stmt, functions);
            Signal::None
        }
        Rule::cast => {
            handle_cast(stmt, scope, functions, output);
            Signal::None
        }
        Rule::bestow => Signal::Return(handle_bestow(stmt, scope, functions, output)),
        Rule::yield_stmt => Signal::Return(handle_yield(stmt, scope, functions, output)),
        Rule::dispel => Signal::Break,
        Rule::portal => Signal::Continue,
        _ => Signal::None, // Skip unhandled types like comments or EOI
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

fn handle_curse(pair: pest::iterators::Pair<Rule>, output: &mut OutputCollector) {
    let message = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    output.eprintln(&format!("CURSE: {}", message));
    std::process::exit(1);
}

fn handle_summon(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> Signal {
    let expr = pair.into_inner().next().unwrap();
    let value = evaluate_expression(expr, scope, functions, output);
    Signal::Error(value.to_display_string())
}

fn handle_invoke_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let mut inner = pair.into_inner();
    let invoke_block = inner.next().unwrap();
    let seal_block = inner.next().unwrap();

    let invoke_statements: Vec<_> = invoke_block.into_inner().collect();
    for stmt in invoke_statements {
        match match_incantation(stmt, scope, functions, output) {
            Signal::Error(msg) => {
                return handle_seal_block(seal_block, &msg, scope, functions, output);
            }
            Signal::Return(val) => return Signal::Return(val),
            Signal::Break => return Signal::Break,
            Signal::Continue => return Signal::Continue,
            Signal::None => {}
        }
    }

    Signal::None
}

fn handle_seal_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    error_msg: &str,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let mut inner = pair.into_inner();

    let first = inner.next().unwrap();
    let (error_var, block) = if first.as_rule() == Rule::IDENT {
        let block = inner.next().unwrap();
        (Some(first.as_str().to_string()), block)
    } else {
        (None, first)
    };

    if let Some(var_name) = error_var {
        scope.insert(var_name, ExprValue::String(error_msg.to_string()));
    }

    let statements: Vec<_> = block.into_inner().collect();
    for stmt in statements {
        match match_incantation(stmt, scope, functions, output) {
            Signal::Return(val) => return Signal::Return(val),
            Signal::Break => return Signal::Break,
            Signal::Continue => return Signal::Continue,
            Signal::Error(msg) => return Signal::Error(msg),
            Signal::None => {}
        }
    }

    Signal::None
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
) {
    let raw = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    let command = interpolate(raw, scope);
    let cmd_output = shell_command(&command, shell_override).output();

    match cmd_output {
        Ok(cmd_output) => {
            let code = cmd_output.status.code().unwrap_or(1);
            scope.insert("_exit".to_string(), ExprValue::Number(code as f64));

            if !cmd_output.stdout.is_empty() {
                output.print(&String::from_utf8_lossy(&cmd_output.stdout));
            }
            if !cmd_output.stderr.is_empty() {
                output.eprint(&String::from_utf8_lossy(&cmd_output.stderr));
            }
            if !cmd_output.status.success() {
                output.eprintln(&format!("Command failed with exit code {}", code));
            }
        }
        Err(e) => {
            output.eprintln(&format!("Failed to evoke command: {}", e));
        }
    }
}

fn handle_scry_chain<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let mut inner = pair.into_inner();
    let scry_cond = inner.next().unwrap();
    let scry_block = inner.next().unwrap();

    let statements: Vec<_> = scry_block.into_inner().collect();

    if eval_condition(scry_cond, scope, functions, output) {
        for stmt in statements {
            let signal = match_incantation(stmt, scope, functions, output);
            if !matches!(signal, Signal::None) {
                return signal;
            }
        }
        return Signal::None;
    }

    for morph in inner {
        if morph.as_rule() == Rule::morph_block {
            let mut morph_inner = morph.into_inner();
            let morph_cond = morph_inner.next().unwrap();
            let morph_block = morph_inner.next().unwrap();

            if eval_condition(morph_cond, scope, functions, output) {
                for stmt in morph_block.into_inner() {
                    let signal = match_incantation(stmt, scope, functions, output);
                    if !matches!(signal, Signal::None) {
                        return signal;
                    }
                }
                return Signal::None;
            }
        } else if morph.as_rule() == Rule::lest_block {
            for stmt in morph.into_inner() {
                let signal = match_incantation(stmt, scope, functions, output);
                if !matches!(signal, Signal::None) {
                    return signal;
                }
            }
            return Signal::None;
        }
    }
    Signal::None
}

// ─── Loops ───────────────────────────────────────────────────────────

fn handle_loop_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let block = pair.into_inner().next().unwrap();
    let statements: Vec<_> = block.into_inner().collect();

    'outer: for _ in 0..3 {
        for stmt in &statements {
            match match_incantation(stmt.clone(), scope, functions, output) {
                Signal::Break => break 'outer,
                Signal::Continue => break,
                Signal::Return(val) => return Signal::Return(val),
                Signal::Error(msg) => return Signal::Error(msg),
                Signal::None => {}
            }
        }
    }
    Signal::None
}

fn handle_channel_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let mut inner = pair.into_inner();
    let cond = inner.next().unwrap();
    let block = inner.next().unwrap();

    let statements: Vec<_> = block.into_inner().collect();

    let mut iteration_count = 0;
    'outer: while eval_condition(cond.clone(), scope, functions, output) {
        iteration_count += 1;

        if iteration_count > 10 {
            output.eprintln(
                "Channel loop exceeded 10 iterations, breaking to prevent infinite loop",
            );
            break;
        }

        for stmt in &statements {
            match match_incantation(stmt.clone(), scope, functions, output) {
                Signal::Break => break 'outer,
                Signal::Continue => break,
                Signal::Return(val) => return Signal::Return(val),
                Signal::Error(msg) => return Signal::Error(msg),
                Signal::None => {}
            }
        }
    }
    Signal::None
}

fn handle_chant_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
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
        _ => return Signal::None,
    };

    if step_num == 0 {
        output.eprintln("Step cannot be zero");
        return Signal::None;
    }

    if step_num > 0 {
        let mut current = start_num;
        'outer: while current < end_num {
            scope.insert(loop_var.clone(), ExprValue::Number(current as f64));

            for stmt in &statements {
                match match_incantation(stmt.clone(), scope, functions, output) {
                    Signal::Break => break 'outer,
                    Signal::Continue => break,
                    Signal::Return(val) => return Signal::Return(val),
                    Signal::Error(msg) => return Signal::Error(msg),
                    Signal::None => {}
                }
            }

            current += step_num;
        }
    } else {
        let mut current = start_num;
        'outer: while current > end_num {
            scope.insert(loop_var.clone(), ExprValue::Number(current as f64));

            for stmt in &statements {
                match match_incantation(stmt.clone(), scope, functions, output) {
                    Signal::Break => break 'outer,
                    Signal::Continue => break,
                    Signal::Return(val) => return Signal::Return(val),
                    Signal::Error(msg) => return Signal::Error(msg),
                    Signal::None => {}
                }
            }

            current += step_num;
        }
    }
    Signal::None
}

fn handle_recite_block<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) -> Signal {
    let mut inner = pair.into_inner();
    let loop_var = inner.next().unwrap().as_str().to_string();
    let list_expr = inner.next().unwrap();
    let block = inner.next().unwrap();
    let statements: Vec<_> = block.into_inner().collect();

    let list_val = evaluate_factor(list_expr, scope, functions, output);

    macro_rules! run_loop {
        ($iter:expr) => {
            'outer: for item in $iter {
                scope.insert(loop_var.clone(), item);
                for stmt in &statements {
                    match match_incantation(stmt.clone(), scope, functions, output) {
                        Signal::Break => break 'outer,
                        Signal::Continue => break,
                        Signal::Return(val) => return Signal::Return(val),
                        Signal::Error(msg) => return Signal::Error(msg),
                        Signal::None => {}
                    }
                }
            }
        };
    }

    match list_val {
        ExprValue::String(s) => {
            if s.trim().is_empty() {
                return Signal::None;
            }
            let items: Vec<ExprValue> = s
                .split(',')
                .map(|s| s.trim())
                .filter(|item| !item.is_empty())
                .map(|item| ExprValue::String(item.to_string()))
                .collect();
            run_loop!(items.into_iter());
        }
        ExprValue::Number(n) => {
            let items: Vec<ExprValue> = (0..(n as i32))
                .map(|i| ExprValue::Number(i as f64))
                .collect();
            run_loop!(items.into_iter());
        }
        ExprValue::Boolean(_b) => {
            run_loop!(std::iter::once(ExprValue::Number(0.0)));
        }
        ExprValue::List(l) => {
            run_loop!(l.into_iter());
        }
        ExprValue::Map(m) => {
            let keys: Vec<ExprValue> = m.into_keys().map(ExprValue::String).collect();
            run_loop!(keys.into_iter());
        }
    }
    Signal::None
}

fn handle_enchant<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();

    let next_pair = inner.next().unwrap();
    let (params, body_pair) = if next_pair.as_rule() == Rule::param_list {
        let params = next_pair
            .into_inner()
            .map(|p| p.as_str().to_string())
            .collect();
        let body_pair = inner.next().unwrap();
        (params, body_pair)
    } else {
        (Vec::new(), next_pair)
    };

    let body: Vec<_> = body_pair.into_inner().collect();
    let func = FunctionDef { params, body };
    functions.insert(name, func);
}

fn handle_cast<'i>(
    pair: pest::iterators::Pair<'i, Rule>,
    parent_scope: &mut HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef<'i>>,
    output: &mut OutputCollector,
) {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();
    let args_pair = inner.next();

    let args = resolve_args(args_pair, parent_scope, functions, output);

    if builtins::is_builtin(name) {
        let string_args: Vec<String> = args.iter().map(|a| a.to_display_string()).collect();
        match builtins::call_builtin(name, string_args, output) {
            Ok(result) => match result {
                builtins::BuiltinValue::None => {}
                builtins::BuiltinValue::Boolean(true) => {}
                _ => output.println(&format!("{}", result)),
            },
            Err(e) => output.eprintln(&format!("Error calling {}: {}", name, e)),
        }
    } else if let Some(func) = functions.get(name) {
        let mut scope = parent_scope.clone();
        for (param, arg) in func.params.iter().zip(args.into_iter()) {
            scope.insert(param.clone(), arg);
        }
        for stmt in func.body.clone() {
            if let Signal::Return(_val) = match_incantation(stmt, &mut scope, functions, output) {
                return;
            }
        }
    } else {
        output.eprintln(&format!("Unknown function: {}", name));
    }
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

fn evaluate_function_call(
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
            if let Signal::Return(val) =
                match_incantation(stmt, &mut func_scope, functions, output)
            {
                return val;
            }
        }

        ExprValue::String("".to_string())
    } else {
        output.eprintln(&format!("Unknown function: {}", name));
        ExprValue::String("".to_string())
    }
}

fn evaluate_method_call(
    pair: pest::iterators::Pair<Rule>,
    scope: &HashMap<String, ExprValue>,
    functions: &mut HashMap<String, FunctionDef>,
    output: &mut OutputCollector,
) -> ExprValue {
    let mut inner = pair.into_inner();
    let object_pair = inner.next().unwrap();
    let method_name = inner.next().unwrap().as_str();
    let args_pair = inner.next();

    let object = match object_pair.as_rule() {
        Rule::IDENT => {
            let var_name = object_pair.as_str();
            if let Some(val) = scope.get(var_name) {
                val.clone()
            } else {
                output.eprintln(&format!("Unknown variable: {}", var_name));
                ExprValue::String("".to_string())
            }
        }
        Rule::string => ExprValue::String(process_escape_sequences(
            object_pair.as_str().trim_matches('"'),
        )),
        _ => evaluate_factor(object_pair, scope, functions, output),
    };

    let args = resolve_args(args_pair, scope, functions, output);

    match object {
        ExprValue::String(s) => call_string_method(&s, method_name, args, output),
        ExprValue::List(l) => call_list_method(&l, method_name, args, output),
        ExprValue::Map(m) => call_map_method(&m, method_name, args, output),
        _ => {
            output.eprintln(&format!("Cannot call method on {:?}", object));
            ExprValue::String("".to_string())
        }
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
                Rule::call => evaluate_function_call(inner_value, scope, functions, output),
                Rule::method_call => {
                    evaluate_method_call(inner_value, scope, functions, output)
                }
                Rule::imbue => evaluate_imbue(inner_value, scope, None, output),
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

fn evaluate_imbue(
    pair: pest::iterators::Pair<'_, Rule>,
    scope: &HashMap<String, ExprValue>,
    shell_override: Option<&str>,
    output: &mut OutputCollector,
) -> ExprValue {
    let raw = pair.into_inner().next().unwrap().as_str().trim_matches('"');
    let command = interpolate(raw, scope);

    match shell_command(&command, shell_override).output() {
        Ok(cmd_output) => {
            let stdout = String::from_utf8_lossy(&cmd_output.stdout)
                .trim()
                .to_string();
            ExprValue::String(stdout)
        }
        Err(e) => {
            output.eprintln(&format!("Failed to imbue command: {}", e));
            ExprValue::String("".to_string())
        }
    }
}

// ─── Operators ───────────────────────────────────────────────────────

fn apply_add_op(
    left: ExprValue,
    op: &str,
    right: ExprValue,
    output: &mut OutputCollector,
) -> ExprValue {
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

fn apply_mult_op(
    left: ExprValue,
    op: &str,
    right: ExprValue,
    output: &mut OutputCollector,
) -> ExprValue {
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
            output.eprintln(&format!(
                "{} value must be a number, got string: {}",
                label, s
            ));
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

// ─── String Methods ──────────────────────────────────────────────────

fn call_string_method(
    s: &str,
    method_name: &str,
    args: Vec<ExprValue>,
    output: &mut OutputCollector,
) -> ExprValue {
    match method_name {
        "upper" => ExprValue::String(s.to_uppercase()),
        "lower" => ExprValue::String(s.to_lowercase()),
        "trim" => ExprValue::String(s.trim().to_string()),
        "len" => ExprValue::Number(s.len() as f64),
        "contains" => {
            if let Some(ExprValue::String(substr)) = args.first() {
                ExprValue::Boolean(s.contains(substr.as_str()))
            } else {
                output.eprintln("contains requires a string argument");
                ExprValue::Boolean(false)
            }
        }
        "replace" => {
            if args.len() >= 2 {
                if let (Some(ExprValue::String(from)), Some(ExprValue::String(to))) =
                    (args.get(0), args.get(1))
                {
                    ExprValue::String(s.replace(from.as_str(), to.as_str()))
                } else {
                    output.eprintln("replace requires two string arguments");
                    ExprValue::String(s.to_string())
                }
            } else {
                output.eprintln("replace requires two arguments");
                ExprValue::String(s.to_string())
            }
        }
        "split" => {
            if let Some(ExprValue::String(delim)) = args.first() {
                let parts: Vec<ExprValue> = s
                    .split(delim.as_str())
                    .map(|p| ExprValue::String(p.to_string()))
                    .collect();
                ExprValue::List(parts)
            } else {
                output.eprintln("split requires a string delimiter");
                ExprValue::List(vec![])
            }
        }
        _ => {
            output.eprintln(&format!("Unknown string method: {}", method_name));
            ExprValue::String("".to_string())
        }
    }
}

// ─── List Methods ────────────────────────────────────────────────────

fn call_list_method(
    l: &[ExprValue],
    method_name: &str,
    args: Vec<ExprValue>,
    output: &mut OutputCollector,
) -> ExprValue {
    match method_name {
        "len" => ExprValue::Number(l.len() as f64),
        "first" => l
            .first()
            .cloned()
            .unwrap_or(ExprValue::String("".to_string())),
        "last" => l
            .last()
            .cloned()
            .unwrap_or(ExprValue::String("".to_string())),
        "join" => {
            if let Some(ExprValue::String(delim)) = args.first() {
                let joined: String = l
                    .iter()
                    .map(|v| v.to_display_string())
                    .collect::<Vec<_>>()
                    .join(delim);
                ExprValue::String(joined)
            } else {
                let joined: String = l
                    .iter()
                    .map(|v| v.to_display_string())
                    .collect::<Vec<_>>()
                    .join("");
                ExprValue::String(joined)
            }
        }
        _ => {
            output.eprintln(&format!("Unknown list method: {}", method_name));
            ExprValue::String("".to_string())
        }
    }
}

// ─── Map Methods ─────────────────────────────────────────────────────

fn call_map_method(
    m: &HashMap<String, ExprValue>,
    method_name: &str,
    args: Vec<ExprValue>,
    output: &mut OutputCollector,
) -> ExprValue {
    match method_name {
        "len" => ExprValue::Number(m.len() as f64),
        "keys" => {
            let keys: Vec<ExprValue> = m.keys().map(|k| ExprValue::String(k.clone())).collect();
            ExprValue::List(keys)
        }
        "values" => {
            let values: Vec<ExprValue> = m.values().cloned().collect();
            ExprValue::List(values)
        }
        "has" => {
            if let Some(ExprValue::String(key)) = args.first() {
                ExprValue::Boolean(m.contains_key(key))
            } else {
                output.eprintln("has requires a string key");
                ExprValue::Boolean(false)
            }
        }
        _ => {
            output.eprintln(&format!("Unknown map method: {}", method_name));
            ExprValue::String("".to_string())
        }
    }
}
