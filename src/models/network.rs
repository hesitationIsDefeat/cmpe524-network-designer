use crate::models::link::Link;
use crate::models::node::Node;
use petgraph::graph::{Graph, NodeIndex};

pub struct Network {
    pub graph: Graph<Node, Link, petgraph::Undirected>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            graph: Graph::new_undirected(),
        }
    }

    pub fn add_router(&mut self, node: Node) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_connection(&mut self, a: NodeIndex, b: NodeIndex, link: Link) {
        self.graph.add_edge(a, b, link);
    }

    pub fn total_link_cost(&self) -> f64 {
        self.graph.edge_weights().map(|link| link.cost).sum()
    }

    pub fn is_fully_connected(&self) -> bool {
        petgraph::algo::connected_components(&self.graph) <= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::link::{Link, LinkType};
    use crate::models::node::{Node, NodeType};

    // Helper function to create a dummy node
    fn mock_node(id: usize) -> Node {
        Node::new(id, NodeType::ROUTER, 100.0)
    }

    fn mock_link(id: usize, capacity: f64, cost: f64) -> Link {
        Link::new(id, LinkType::WIRELESS, capacity, cost)
    }

    #[test]
    fn test_network_creation_and_linking() {
        let mut network = Network::new();
        let r1 = network.add_router(mock_node(1));
        let r2 = network.add_router(mock_node(2));

        network.add_connection(r1, r2, mock_link(1, 100.0, 50.0));

        assert_eq!(network.graph.node_count(), 2);
        assert_eq!(network.graph.edge_count(), 1);
    }

    #[test]
    fn test_total_link_cost() {
        let mut network = Network::new();
        let r1 = network.add_router(mock_node(1));
        let r2 = network.add_router(mock_node(2));
        let r3 = network.add_router(mock_node(3));

        network.add_connection(r1, r2, mock_link(1, 100.5, 50.0));
        network.add_connection(r2, r3, mock_link(2, 200.0, 50.5));

        assert_eq!(network.total_link_cost(), 100.5);
    }

    #[test]
    fn test_connectivity_logic() {
        let mut network = Network::new();
        let r1 = network.add_router(mock_node(1));
        let r2 = network.add_router(mock_node(2));
        let r3 = network.add_router(mock_node(3));

        assert!(
            !network.is_fully_connected(),
            "Empty network with multiple nodes should not be connected"
        );

        network.add_connection(r1, r2, mock_link(1, 100.0, 50.0));
        assert!(
            !network.is_fully_connected(),
            "Network with isolated node should not be connected"
        );

        network.add_connection(r2, r3, mock_link(1, 100.0, 50.0));
        assert!(
            network.is_fully_connected(),
            "All nodes reachable, should be connected"
        );
    }
}
