use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./clay/parse/parser.pest"]
pub struct ClayParser;