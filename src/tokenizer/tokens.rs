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

    pub fn print(&self){
        print!("<Token type={:?} value={:?}> ",self.token_type,self.value)
    }


}


#[derive(Debug,PartialEq)]
pub enum CharType{
    Escape,
    WhiteSpace,
    NewLine,
    Letter,
    Digit,
    Symbol,
    Special,
    Unknown,
}

impl CharType{
    pub fn classify_char(c: char)->CharType{
        match c {
            'a'..='z'| 'A'..='Z' => CharType::Letter,
            '0'..='9'=> CharType::Digit,
            '\n'|'\r' => CharType::NewLine,
            '\t'|' ' => CharType::WhiteSpace,
            '#'|'['|']'|'('|')' |'<'|'>'|'{'|'}' => CharType::Special,
            '!'|'@'|'$'|'%'|'^'|'&'|'*'|'-'|'_'|'`'|'\''| '"'|'/'|'|'|',' |'.'|':'|'?'=> CharType::Symbol,
            '\\' => CharType::Escape,
            _ => CharType::Unknown
        }
    }

}




#[derive(Debug,Clone)]
pub enum TokenTypes {
    Error,
    NewLine,
    WhiteSpace,
    Escape,
    Text,
    Header,
    Inline,
    AnchorURLStart,
    AnchorURLEnd,
    AnchorValueStart,
    AnchorValueEnd,
    Emphasis,
    OrderedList,
    Minus,
    UnderScore,
    Image,
    UniqueIDBegin,
    UniqueIDEnd,
    ClassBegin,
    ClassEnd,
}
