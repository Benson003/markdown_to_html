#[derive(Debug)]
pub enum NodeValue{
    Text(String),
    Number(usize),
    None
}


pub struct ASTNode{
    node_type: NodeType,
    value: NodeValue,
    parent: Option<Box<ASTNode>>,
    children: Option<Vec<Box<ASTNode>>>,
    order: usize,
}

impl ASTNode {
    pub fn new(node_type: NodeType, value: NodeValue,parent:ASTNode) -> ASTNode {
        ASTNode{
            node_type:node_type,
            value: value,
            parent: Some(Box::new(parent)),
            children:Some(vec![]) ,
            order: 0
        }
    }

    pub fn append_node(&mut self,ast_node:ASTNode){
        if let Some(children)=&mut self.children{
            children.push(Box::new(ast_node));
            self.order = children.len();
        } 
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
    Bold,
    Highlight,
    CodeBlock,
    Note,
    OrderedList,
    UnorderedList,
    Value,
}
