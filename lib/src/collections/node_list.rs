use core::default::Default;
use crate::Node;

pub struct NodeList<const NODES: usize, NodeID, NodeData> {
    nodes: [Node<NodeID, NodeData>; NODES]
}

impl<const NODES: usize, NodeID, NodeData> NodeList<NODES, NodeID, NodeData> {
    pub const fn new(nodes: [Node<NodeID, NodeData>; NODES]) -> Self {
        Self{nodes: nodes}
    }

    pub fn len(&self) -> usize {
        NODES
    }

    pub fn get(&self, idx: usize) -> Option<&Node<NodeID, NodeData>> {
        if idx < NODES {
            Some(&self.nodes[idx as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Node<NodeID, NodeData>> {
        if idx < NODES {
            Some(&mut self.nodes[idx as usize])
        } else {
            None
        }
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