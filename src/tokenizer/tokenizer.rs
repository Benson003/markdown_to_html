use core::{iter::{Iterator}, option::Option::Some};


use crate::tokenizer::tokens::{ Token, CharType, TokenTypes};



#[derive(Debug)]
pub struct TokenList {
    pub tokens: Vec<Token>,
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
        let mut chars = source.chars().peekable();
        while let Some(&ch) = chars.peek(){
            match CharType::classify_char(ch){
                CharType::WhiteSpace =>{
                    chars.next();
                    self.append_tokens(Token::new(TokenTypes::WhiteSpace,None));
                }
                CharType::Letter => {
                    buffer.clear();
                    while let Some(&c) = chars.peek(){
                        match CharType::classify_char(c){
                            CharType::Letter=>{
                                buffer.push(c);
                                chars.next();
                            }
                            _=>break
                        }
                    }
                    let value = std::mem::take(&mut buffer);
                    self.append_tokens(Token::new(TokenTypes::Text, Some(value)));
                }
                CharType::Special =>{
                    buffer.clear();
                    while let Some(&c) = chars.peek(){
                        if CharType::classify_char(c) != CharType::Special{break;}
                        match c {
                            '#' =>{
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::Header, None));
                            }
                            '[' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::AnchorValueStart, None));
                            }
                            ']' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::AnchorValueEnd, None));
                            }
                            '(' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::AnchorURLStart, None));

                            }
                            ')' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::AnchorURLEnd, None));

                            }
                            '<' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::UniqueIDBegin, None));

                            }
                            '>' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::UniqueIDEnd, None));
                            }
                            '{' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::ClassBegin, None));
                            }
                            '}' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::ClassEnd, None));
                            }
                            _ => break
                        }
                    }
                }

                CharType::Digit => {
                    let mut is_ordered_list = false;
                    let mut num_str = String::new();

                    while let Some(&c) = chars.peek() {
                        match c {
                            '0'..='9' => {
                                num_str.push(c);
                                chars.next(); // consume digit
                            }
                            '.' => {
                                let mut check_iter = chars.clone();
                                check_iter.next(); // '.'
                                if check_iter.next() == Some(' ') && !num_str.is_empty() {
                                    // Confirm ordered list
                                    is_ordered_list = true;
                                    chars.next(); // consume '.'
                                } else {
                                    break; // not an ordered list
                                }
                            }
                            _ => break,
                        }
                    }

                    if is_ordered_list {
                        self.append_tokens(Token::new(TokenTypes::OrderedList, Some(num_str)));
                    } else {
                        self.append_tokens(Token::new(TokenTypes::Text, Some(num_str)));
                    }
                }


                CharType::NewLine => {
                    chars.next();
                    self.append_tokens(Token::new(TokenTypes::NewLine,None));
                }
                CharType::Symbol => {
                    buffer.clear();
                    while let Some(&c) = chars.peek(){
                        if CharType::classify_char(c) != CharType::Symbol{break;}
                        match c {
                            '!' => {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::Image, None));
                            }

                            '*'=>{
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::Emphasis, None));
                            }
                            '`'=> {
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::Inline, None));
                            }
                            '-'=>{
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::Minus, None));
                            }
                            '_'=>{
                                chars.next();
                                self.append_tokens(Token::new(TokenTypes::UnderScore, None));
                            }
                            _=> {
                                chars.next();
                                buffer.push(c);
                                let value = std::mem::take(&mut buffer);
                                self.append_tokens(Token::new(TokenTypes::Text, Some(value)));
                                break;
                            }


                        }

                    }
                }
                CharType::Escape =>{
                    chars.next();
                    if let Some(ch) = chars.next(){
                        buffer.push(ch);
                        let value = std::mem::take(&mut buffer);
                        self.append_tokens(Token::new(TokenTypes::Escape, Some(value)));
                    }
                }
                CharType::Unknown => {
                    buffer.clear();
                    chars.next();
                    buffer.push(ch);
                    let value = std::mem::take(&mut buffer);
                    self.append_tokens(Token::new(TokenTypes::Error, Some(value)));
                }
            }
        }
    }
}
