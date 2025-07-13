use core::option::Option::None;
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Token{
    pub token_type: TokenTypes,
    pub value: Option<String>,
}
impl Token {
    pub fn new(token_type:TokenTypes,value:Option<String>)-> Token{
        Token{
            token_type:token_type,
            value:value,
        }
    }
}

pub enum TokenState{
    Default,
    Escape,
    Value,
    Text,
    Header,
    Inline,
    Anchor,
    Class,
    UniqueID,
    List,
}

#[derive(Debug,Clone)]
pub enum TokenTypes {
    Error,
    Escape,
    Value,
    Text,
    Header,
    Inline,
    AnchorURLStart,
    AnchorURLEnd,
    AnchorValueStart,
    AnchorValueEnd,
    Emphasis,
    OrderedList,
    MinusToken,
    Image,
    UniqueIDBegin,
    UniqueIDEnd,
    ClassBegin,
    ClassEnd,
}
