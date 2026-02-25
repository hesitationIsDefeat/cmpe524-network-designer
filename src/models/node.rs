#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    ROUTER,
}

pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub capacity: f64,
}
