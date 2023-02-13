use std::fmt::{Display, Debug};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NodeId {
    port: i64,
    addr: String,
}

impl NodeId {
    pub fn new(port: i64, addr: String) -> Self {
        Self { port, addr }
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.addr, self.port)
    }
}

impl Debug for NodeId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
