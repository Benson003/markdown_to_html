use crate::tokenizer::tokenizer::TokenList;
use crate::parser::ast::{State, NodeType, ASTNode,NodeValue};



pub struct AST {

    root: ASTNode

}

impl AST {
    fn init ()->AST{ 
        return AST;
    }

    fn parse(token_list :TokenList){
          for token in token_list.tokens { 
           } 
    }

}


