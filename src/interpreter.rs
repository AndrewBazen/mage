use pest::iterators::Pairs;
use crate::parser::Rule;
use std::collections::HashMap;

pub fn interpret(pairs: Pairs<Rule>) {
    let mut scope = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::statement => {
                let mut inner = pair.into_inner();
                let stmt = inner.next().unwrap();

                match stmt.as_rule() {
                    Rule::conjure => handle_conjure(stmt, &mut scope),
                    Rule::incant => handle_incant(stmt, &scope),
                    _ => unreachable!("Unknown statement: {:?}", stmt),
                }
            }
            _ => {}
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

    // Simple interpolation: replace $var with scope value
    let interpolated = interpolate(raw, scope);
    println!("{}", interpolated);
}

fn interpolate(text: &str, scope: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '$' {
            // Start of variable
            let mut var = String::new();
            while let Some(c) = chars.peek() {
                if c.is_alphanumeric() || *c == '_' {
                    var.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }

            if let Some(value) = scope.get(&var) {
                result.push_str(value);
            } else {
                result.push_str(&format!("${}", var)); // fallback to original
            }
        } else {
            result.push(ch);
        }
    }

    result
}
