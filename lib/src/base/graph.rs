use super::Node;
use super::Edge;

pub struct Graph<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    pub next_node: NodeID,
    pub next_edge: EdgeID,
    pub nodes: [Node<NodeID, NodeData>; NODES],
    pub edges: [Edge<NodeID, EdgeID, EdgeData>; EDGES]
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
    where   NodeID: core::ops::Add<usize, Output = NodeID> + core::ops::Rem<usize, Output = NodeID> + core::slice::SliceIndex<[Node<NodeID, NodeData>], Output = Node<NodeID, NodeData>> + core::marker::Copy,
            EdgeID: core::ops::Add<usize, Output = EdgeID> + core::ops::Rem<usize, Output = EdgeID> + core::slice::SliceIndex<[Edge<NodeID, EdgeID, EdgeData>], Output = Edge<NodeID, EdgeID, EdgeData>> + core::marker::Copy,
    {

    pub fn find_node(&self, at: NodeID) -> &Node<NodeID, NodeData> {
        &self.nodes[at]
    }

    pub fn find_edge(&self, at: EdgeID) -> &Edge<NodeID, EdgeID, EdgeData> {
        &self.edges[at]
    }

    pub fn next_node(&mut self) -> &Node<NodeID, NodeData> {
        self.next_node = (self.next_node + 1) % NODES;
        self.find_node(self.next_node)
    }

    pub fn next_edge(&mut self) -> &Edge<NodeID, EdgeID, EdgeData> {
        self.next_edge = (self.next_edge + 1) % EDGES;
        self.find_edge(self.next_edge)
    }
}