use core::default::Default;

#[derive(Clone, Copy)]
pub struct Node<NodeID, NodeData> {
    pub id: NodeID,
    pub data: NodeData,
}

impl<NodeID, NodeData> Node<NodeID, NodeData> {
    pub const fn new(id: NodeID, data: NodeData) -> Self {
        Self{id: id, data: data}
    }
}

impl<NodeID, NodeData> Default for Node<NodeID, NodeData>
    where   NodeID: Default,
            NodeData: Default
{
    fn default() -> Self {
        Self {
            id: Default::default(),
            data: Default::default()
        }
    }
}