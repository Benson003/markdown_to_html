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

impl ASTNode {
    pub fn new(node_type: NodeType, value: NodeValue,parent: Option<ASTNode>) -> ASTNode {
        ASTNode{
            node_type:node_type,
            value: value,
            parent: parent,
            children: vec![],
            order: 1
        }
    }

    pub fn append_node(&mut self,ast_node:ASTNode){
        self.children.append(ast_node);
        self.order = self.children.len() + 1;
    }
}

#[derive(Debug)]
pub enum NodeType {
    Root,
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
