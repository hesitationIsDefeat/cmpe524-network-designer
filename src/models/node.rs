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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(0, NodeType::ROUTER, 10.0);

        assert_eq!(node.id, 0);
        assert_eq!(node.node_type, NodeType::ROUTER);
        assert_eq!(node.capacity, 10.0);
    }

    #[test]
    fn test_node_comparison() {
        let first_node = Node::new(0, NodeType::ROUTER, 10.0);
        let second_node = Node::new(0, NodeType::ROUTER, 20.0);

        assert!(first_node.id < second_node.id);
        assert_eq!(first_node.node_type, second_node.node_type);
        assert!(first_node.capacity < second_node.capacity);
    }
}
