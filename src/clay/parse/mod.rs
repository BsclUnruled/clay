use std::fmt::Debug;

use pest::Parser;
use pest_derive::Parser;

use super::vm::{Ast, ToRun};

#[derive(Parser)]
#[grammar = "./clay/parse/parser.pest"]
pub struct ClayParser;

pub fn parse(input: &str)->Result<ToRun,pest::error::Error<Rule>>{
    let pairs = ClayParser::parse(Rule::script, input);
    pairs.map(|_|Ast::None)
}