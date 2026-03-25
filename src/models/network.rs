//! This file contains the required structures and relevant unit tests for the  network representation.

use crate::models::link::Link;
use crate::models::node::Node;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;

/// Represents network topology
pub struct Network {
    /// Undirected graph with Node instances as nodes and Link instances as edges
    pub graph: StableGraph<Node, Link, petgraph::Undirected>,
}

impl Network {
    /// Creates a Network with specific parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// let network = Network::new();
    /// ```
    pub fn new() -> Self {
        Self {
            graph: StableGraph::default(),
        }
    }

    /// Creates a Network with specific capacity parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// let network = Network::with_capacity(20, 40);
    /// ```
    pub fn with_capacity(node_capacity: usize, link_capacity: usize) -> Self {
        Self {
            graph: StableGraph::with_capacity(node_capacity, link_capacity),
        }
    }

    /// Adds a [`Node`] to the [`Network`] and returns the [`NodeIndex`].
    ///
    /// This index is required to add [`Link`]s between [`Node`]s
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// # use cmpe524_network_designer::models::node::{Node, NodeType, Point3D};
    /// let mut network = Network::new();
    /// let node = Node::new(1, NodeType::ROUTER, 100.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,});
    ///
    /// let node_index = network.add_node(node);
    ///
    /// assert_eq!(network.graph.node_count(), 1);
    /// ```
    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.graph.add_node(node)
    }

    /// Creates an undirected connection between two nodes in the network.
    ///
    /// The link is represented by a [`Link`] struct which typically contains
    /// physical properties like capacity and cost.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// # use cmpe524_network_designer::models::node::{Node, NodeType, Point3D};
    /// # use cmpe524_network_designer::models::link::{Link, LinkType};
    /// let mut network = Network::new();
    /// let a = network.add_node(Node::new(1, NodeType::ROUTER, 100.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    /// let b = network.add_node(Node::new(2, NodeType::ROUTER, 100.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    ///
    /// network.add_link(a, b, Link::new(1, LinkType::WIRELESS, 100.0, 50.0));
    /// assert_eq!(network.graph.edge_count(), 1);
    /// ```
    pub fn add_link(&mut self, a: NodeIndex, b: NodeIndex, link: Link) {
        self.graph.add_edge(a, b, link);
    }

    /// Calculates the sum of the costs of all existing links in the topology.
    ///
    /// This is often a primary component of the objective function used
    /// in the optimization process.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// # use cmpe524_network_designer::models::node::{Node, NodeType, Point3D};
    /// # use cmpe524_network_designer::models::link::{Link, LinkType};
    /// let mut network = Network::new();
    /// let a = network.add_node(Node::new(1, NodeType::ROUTER, 10.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    /// let b = network.add_node(Node::new(2, NodeType::ROUTER, 10.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    ///
    /// network.add_link(a, b, Link::new(1, LinkType::WIRELESS, 100.0, 50.5));
    /// assert_eq!(network.total_link_cost(), 50.5);
    /// ```
    pub fn total_link_cost(&self) -> f64 {
        self.graph.edge_weights().map(|link| link.cost).sum()
    }

    /// Determines if every node in the network is reachable from every other node.
    ///
    /// A network is considered fully connected if it contains at most one
    /// connected component. An empty network or a single-node network
    /// will return `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::network::Network;
    /// # use cmpe524_network_designer::models::node::{Node, NodeType, Point3D};
    /// # use cmpe524_network_designer::models::link::{Link, LinkType};
    /// let mut network = Network::new();
    /// let a = network.add_node(Node::new(1, NodeType::ROUTER, 10.0, Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    /// let b = network.add_node(Node::new(2, NodeType::ROUTER, 10.0, Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,}));
    ///
    /// // Initially disconnected
    /// assert!(!network.is_fully_connected());
    ///
    /// // Becomes connected after adding a link
    /// network.add_link(a, b, Link::new(1, LinkType::WIRELESS, 100.0, 50.0));
    /// assert!(network.is_fully_connected());
    /// ```
    pub fn is_fully_connected(&self) -> bool {
        if self.graph.node_count() == 0 {
            return false;
        }

        let components = petgraph::algo::tarjan_scc(&self.graph);
        components.len() == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::link::{Link, LinkType};
    use crate::models::node::{Node, NodeType, Point3D};

    // Helper function to create a dummy node
    fn mock_node(id: usize) -> Node {
        Node::new(
            id,
            NodeType::ROUTER,
            100.0,
            Point3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3D {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
        )
    }

    fn mock_link(id: usize, capacity: f64, cost: f64) -> Link {
        Link::new(id, LinkType::WIRELESS, capacity, cost)
    }

    #[test]
    fn test_network_creation_and_linking() {
        let mut network = Network::new();
        let r1 = network.add_node(mock_node(1));
        let r2 = network.add_node(mock_node(2));

        network.add_link(r1, r2, mock_link(1, 100.0, 50.0));

        assert_eq!(network.graph.node_count(), 2);
        assert_eq!(network.graph.edge_count(), 1);
    }

    #[test]
    fn test_network_with_capacity() {
        let mut network = Network::with_capacity(2, 1);

        let r1 = network.add_node(mock_node(1));
        let r2 = network.add_node(mock_node(2));

        network.add_link(r1, r2, mock_link(1, 100.0, 50.0));

        assert_eq!(network.graph.node_count(), 2);
        assert_eq!(network.graph.edge_count(), 1);
    }

    #[test]
    fn test_total_link_cost() {
        let mut network = Network::new();
        let r1 = network.add_node(mock_node(1));
        let r2 = network.add_node(mock_node(2));
        let r3 = network.add_node(mock_node(3));

        network.add_link(r1, r2, mock_link(1, 100.5, 50.0));
        network.add_link(r2, r3, mock_link(2, 200.0, 50.5));

        assert_eq!(network.total_link_cost(), 100.5);
    }

    #[test]
    fn test_connectivity_logic() {
        let mut network = Network::new();
        let r1 = network.add_node(mock_node(1));
        let r2 = network.add_node(mock_node(2));
        let r3 = network.add_node(mock_node(3));

        assert!(
            !network.is_fully_connected(),
            "Empty network with multiple nodes should not be connected"
        );

        network.add_link(r1, r2, mock_link(1, 100.0, 50.0));
        assert!(
            !network.is_fully_connected(),
            "Network with isolated node should not be connected"
        );

        network.add_link(r2, r3, mock_link(1, 100.0, 50.0));
        assert!(
            network.is_fully_connected(),
            "All nodes reachable, should be connected"
        );
    }
}
