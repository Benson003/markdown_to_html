#[derive(Debug)]
pub enum ASTNode {
    Text(String),
    Header{
        level:usize,
        content:Vec<ASTNode>
    },
    
}
