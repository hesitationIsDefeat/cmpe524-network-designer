#[derive(Debug, Clone, PartialEq)]
pub enum LinkType {
    ETHERNET,
    WIRELESS,
}

pub struct Link {
    pub id: usize,
    pub link_type: LinkType,
    pub capacity: f64,
}

impl Link {
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
