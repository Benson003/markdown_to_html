use std::cell::RefCell;

#[derive(Debug)]
pub enum NodeValue{
    Text(String),
    Number(usize),
    None
}

#[derive(Debug)]
pub struct ASTNode{
    node_type: NodeType,
    value: NodeValue,
    parent: Option<ASTNode>,
    children: Vec<Option<ASTNode>>,
    order: usize,
}

#[derive(Debug)]
pub enum NodeType {
    Header,
    Text,
    UniqueID,
    Class,
    Image,
    Anchor,
    Italic,
    Bold,
    Highlight,
    CodeBlock,
    Note,
    UnorderedList,
    OrderedList,
    ListItem,
    Value,
}

#[derive(Debug)]
pub enum State{
    Default,
    Header,
    Text,
    Value,
    Bold,
    Highlight,
    CodeBlock,
    Note,
    OrderedList,
    UnorderedList,
    Value,
}
