#[derive(Clone, Copy)]
pub struct Node<NodeID, NodeData> {
    pub id: NodeID,
    pub data: NodeData,
}