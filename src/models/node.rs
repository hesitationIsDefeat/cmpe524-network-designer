//! This file contains the required structures and relevant unit tests for the node of a network.
use crate::models::geo::Point3D;

#[derive(Debug, Clone, PartialEq)]
/// Represents the type of a Node
pub enum NodeType {
    /// UAV (server) node
    UAV,
    /// User (client) node
    USER,
}

/// Represents the basic unit in network topology
pub struct Node {
    /// Unique identifier
    pub id: usize,
    /// Type (e.g. UAV)
    pub node_type: NodeType,
    /// Maximum work capacity, the unit depends on the network type (e.g. Gbps, pps, tpm)
    pub capacity: f64,
    /// Location of the Node on 3D space (in meters)
    pub location: Point3D,
    /// Maximum velocity of the Node (in meters)
    pub velocity: Point3D,
}

impl Node {
    /// Creates a Node with specific parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cmpe524_network_designer::models::node::{Node, NodeType};
    /// # use cmpe524_network_designer::models::geo::Point3D;
    /// let node = Node::new(0, NodeType::UAV, 10.0,Point3D {x: 0.0,y: 0.0,z: 0.0,}, Point3D {x: 1.0,y: 1.0,z: 0.0,});
    /// ```
    pub fn new(
        id: usize,
        node_type: NodeType,
        capacity: f64,
        location: Point3D,
        velocity: Point3D,
    ) -> Self {
        Self {
            id,
            node_type,
            capacity,
            location,
            velocity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::new(
            0,
            NodeType::UAV,
            10.0,
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
        );

        assert_eq!(node.id, 0);
        assert_eq!(node.node_type, NodeType::UAV);
        assert_eq!(node.capacity, 10.0);
    }

    #[test]
    fn test_node_comparison() {
        let first_node = Node::new(
            0,
            NodeType::UAV,
            10.0,
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
        );
        let second_node = Node::new(
            1,
            NodeType::UAV,
            20.0,
            Point3D {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Point3D {
                x: 2.0,
                y: 2.0,
                z: 0.0,
            },
        );

        assert!(first_node.id < second_node.id);
        assert_eq!(first_node.node_type, second_node.node_type);
        assert!(first_node.capacity < second_node.capacity);
        assert!(first_node.location.x < second_node.location.x);
        assert!(first_node.location.y < second_node.location.y);
        assert!(first_node.velocity.x < second_node.velocity.x);
        assert!(first_node.velocity.x < second_node.velocity.x);
    }
}
