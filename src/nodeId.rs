use std::{fmt::{Display, Debug}, mem::size_of};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct NodeId {
    port: i64,
    addr: String,
}

impl NodeId {
    pub fn new(port: i64, addr: String) -> Self {
        Self { port, addr }
    }

    pub fn get_bytes_size(&self) -> usize {
        return size_of::<i64>() + self.addr.len();
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
