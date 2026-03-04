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
        Self(id, link_type, capacity)
    }
}
