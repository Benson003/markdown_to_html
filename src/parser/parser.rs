use crate::tokenizer::tokenizer::TokenList;
use crate::parser::ast::{State, NodeType, ASTNode,NodeValue};
use crate::tokenizer::tokens::{TokenTypes,Token};



pub struct AST {

    root: ASTNode

}

impl AST {

    pub fn parse(token_list :TokenList){
        let mut state = State::Default;
        for token in token_list.tokens {
            match state {
                State::Default => {
                    switch_state(&mut state,&token);
                }

                State::Text=> {
                }
                State::Bold=> {
                }
                State::Value=> {
                }
                State::Highlight=> {
                }
                State::Note=> {
                }
                State::CodeBlock=> {
                }
                State::OrderedList=> {
                }State::UnorderedList=>{

                }
                State::Header=>{
                }


            }
        }
    }

}

fn switch_state(state:&mut State,token:&Token){
    match token.token_type{
        TokenTypes::Error =>{}
        TokenTypes::NewLine =>{}
        TokenTypes::WhiteSpace =>{}
        TokenTypes::Escape => {}
        TokenTypes::Text =>{}
        TokenTypes::Header =>{}
        TokenTypes::Inline =>{}
        TokenTypes::AnchorURLStart =>{}
        TokenTypes::AnchorURLEnd =>{}
        TokenTypes::AnchorValueStart =>{}
        TokenTypes::AnchorValueEnd =>{}
        TokenTypes::Emphasis =>{}
        TokenTypes::OrderedList =>{}
        TokenTypes::Minus =>{}
        TokenTypes::UnderScore =>{}
        TokenTypes::Image => {}
        TokenTypes::UniqueIDBegin =>{}
        TokenTypes::UniqueIDEnd => {}
        TokenTypes::ClassBegin => {}
        TokenTypes::ClassEnd => {}

    }
}
