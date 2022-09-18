use core::default::Default;
use core::ops::Index;
use core::ops::IndexMut;
use crate::Node;

pub struct NodeList<const NODES: usize, NodeID, NodeData> {
    nodes: [Node<NodeID, NodeData>; NODES]
}

impl<const NODES: usize, NodeID, NodeData> NodeList<NODES, NodeID, NodeData>
    where   NodeID: core::marker::Copy,
            NodeData: core::marker::Copy
{
    pub const fn new(node: Node<NodeID, NodeData>) -> Self {
        Self{nodes: [node; NODES]}
    }
}

impl<const NODES: usize, NodeID, NodeData> Index<usize> for NodeList<NODES, NodeID, NodeData> {
    type Output = Node<NodeID, NodeData>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.nodes[idx as usize]
    }
}

impl<const NODES: usize, NodeID, NodeData> IndexMut<usize> for NodeList<NODES, NodeID, NodeData> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.nodes[idx as usize]
    }
}

impl<const NODES: usize, NodeID, NodeData> Default for NodeList<NODES, NodeID, NodeData>
    where   NodeID: Default,
            NodeData: Default
{
    fn default() -> Self {
        Self{nodes: [(); NODES].map(|_| Default::default())}
    }
}