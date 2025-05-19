#[macro_use]
extern crate pest_derive;

pub mod parser;
pub mod interpreter;

use pest::Parser;
use pest::iterators::Pairs;
use crate::interpreter::interpret;
use crate::parser::MageParser;

pub fn run(source: &str) -> Result<(), String> {
    match MageParser::parse(crate::Rule::program, source) {
        Ok(pairs) => {
            interpret(pairs);
            Ok(())
        }
        Err(err) => Err(format!("Parse error: {}", err)),
    }
}
