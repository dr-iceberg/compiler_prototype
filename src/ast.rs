#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}
