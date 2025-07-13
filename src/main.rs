mod tokenizer;
mod cmd;

use crate::tokenizer::tokenizer::TokenList;
use crate::cmd::cmd::load_file_from_args;

fn main(){
    let source = load_file_from_args().unwrap();
    let mut tokens =TokenList::new();
    tokens.tokenize(source.as_str());
}
