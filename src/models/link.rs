//! This file contains the required structures and relevant unit tests for the link of a network.

#[derive(Debug, Clone, PartialEq)]
/// Represents the type of a Link
pub enum LinkType {
    /// Represents physical connection through a cable
    ETHERNET,
    /// Represents non-physical connection through air
    WIRELESS,
}
/// Represents node connections in the network topology
pub struct Link {
    /// Unique identifier
    pub id: usize,
    /// Type (e.g. ETHERNET, WIRELESS)
    pub link_type: LinkType,
    /// Maximum work capacity, the unit depends on the network type (e.g. Gbps, pps, tpm)
    pub capacity: f64,
}

impl Link {
    /// Creates a Link with specific parameters.
    ///
    /// # Examples
    ///
    /// ```
    /// use cmpe524_network_designer::models::link::{Link, LinkType};
    /// let link = Link::new(0, LinkType::ETHERNET, 10.0);
    /// ```
    pub fn new(id: usize, link_type: LinkType, capacity: f64) -> Self {
        Self {
            id,
            link_type,
            capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_link_creation() {
        let link: Link = Link::new(0, LinkType::ETHERNET, 10.0);

        assert_eq!(link.id, 0);
        assert_eq!(link.link_type, LinkType::ETHERNET);
        assert_eq!(link.capacity, 10.0);
    }

    #[test]
    pub fn test_link_comparison() {
        let first_link: Link = Link::new(0, LinkType::ETHERNET, 10.0);
        let second_link: Link = Link::new(1, LinkType::WIRELESS, 15.0);

        assert!(first_link.id < second_link.id);
        assert_ne!(first_link.link_type, second_link.link_type);
        assert!(first_link.capacity < second_link.capacity);
    }
}
