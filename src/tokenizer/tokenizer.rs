use core::{char, default::Default, fmt, iter::Iterator, option::Option::{None, Some}};

use crate::tokenizer::tokens::{self, Token, TokenState, TokenTypes};

#[derive(Debug)]
pub struct TokenList {
    tokens: Vec<Token>,
}

impl  TokenList {
    pub fn new() -> TokenList{
        TokenList{
            tokens: Vec::new(),
        }
    }
    fn append_tokens(&mut self,token:Token){
        self.tokens.push(token);
    }
    pub fn print_tokens(&self){
        println!("Tokens: {:?}", self.tokens);
    }
    pub fn tokenize(&mut self,source :&str){
        let mut buffer = String::new();
        let mut chars =source.chars().peekable();
        let mut state: TokenState = TokenState::Default;
        while let Some(&ch)= chars.peek(){
            match state{
                TokenState::Default=>{
                    switch_state(self, &ch, &mut state, &mut buffer, &mut chars);
                }
                TokenState::Escape=>{

                }
                TokenState::Text=>{
                    buffer.clear();
                    while let Some(&c)= chars.peek() {


                    }

                }
                TokenState::Value=>{

                }
                TokenState::Header=>{

                }
                TokenState::Anchor=>{

                }
                TokenState::Inline=> {

                }
                TokenState::Class=>{

                }
                TokenState::UniqueID =>{

                }
                TokenState::List=>{

                }
            }
        }


    }
}



fn manipulate(base_state:&mut TokenState,token_list:&mut TokenList,token:Token,,buffer :&mut String,chars:&mut impl Iterator<Item = char>,function :fn()){
    buffer.clear();
    while let Some(&c) = chars.peekable().peek(){
        function();
        buffer.push(c);
        chars.next();
    }
    token_list.append_tokens(token);
    base_state = end_state;
}

fn switch_state(token_list:&mut TokenList,ch :&char,state :&mut TokenState,buffer :&mut String, chars:&mut impl Iterator<Item = char>){
    match ch {
                        '#' =>{
                            buffer.clear();
                            chars.next();
                            *state = TokenState::Header;
                        }
                        '['=>{
                            buffer.clear();
                            chars.next();
                            *state = TokenState::Anchor;
                        }
                        '*' =>{
                            chars.next();
                            *state = TokenState::Inline;
                        }
                        '\n'|' '| '.'=>{
                            chars.next();
                        }
                        'a'..='z'|'A'..='Z' =>{
                            buffer.clear();
                            *state = TokenState::Text;
                        }

                        _ =>{

                            token_list.append_tokens(Token::new(TokenTypes::Error, Some(ch.to_string())));
                            print!("Falied to parse: {:?}\n ",ch);
                            panic!("Invalid token");

                        }
                    }
}
