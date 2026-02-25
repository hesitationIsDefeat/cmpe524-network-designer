#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    ROUTER,
}

pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub capacity: f64,
}

impl Node {
    pub fn new(id: usize, node_type: NodeType, capacity: f64) -> Self {
        Self {
            id,
            node_type,
            capacity,
        }
    }
}
