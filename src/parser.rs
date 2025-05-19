use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MageParser;

pub use self::Rule;
