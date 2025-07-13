mod tokenizer;
mod cmd;
mod parser;

use crate::tokenizer::tokenizer::TokenList;
use crate::cmd::cmd::load_file_from_args;
use crate::parser::parser;

fn main(){
    let source = load_file_from_args().unwrap();
    //let source = String::from("## Hello World");
    let mut tokens =TokenList::new();
    tokens.tokenize(source.as_str());
    tokens.print_tokens();
}
