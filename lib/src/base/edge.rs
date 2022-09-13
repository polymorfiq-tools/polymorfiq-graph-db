#[derive(Clone, Copy)]
pub struct Edge<NodeID, EdgeID, EdgeData> {
    pub id: EdgeID,
    pub a: NodeID,
    pub b: NodeID,
    pub data: EdgeData,
}