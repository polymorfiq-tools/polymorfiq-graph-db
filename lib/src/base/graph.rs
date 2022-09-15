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

    pub fn node_count(&self) -> usize {
        NODES
    }

    pub fn edge_count(&self) -> usize {
        EDGES
    }

    pub fn next_node_id(&mut self) -> NodeID {
        self.next_node = (self.next_node + 1) % NODES;
        self.next_node
    }

    pub fn next_edge_id(&mut self) -> EdgeID {
        self.next_edge = (self.next_edge + 1) % EDGES;
        self.next_edge
    }

    pub fn find_node(&self, at: NodeID) -> &Node<NodeID, NodeData> {
        &self.nodes[at]
    }

    pub fn find_mutable_node(&mut self, at: NodeID) -> &mut Node<NodeID, NodeData> {
        &mut self.nodes[at]
    }

    pub fn find_edge(&self, at: EdgeID) -> &Edge<NodeID, EdgeID, EdgeData> {
        &self.edges[at]
    }

    pub fn find_mutable_edge(&mut self, at: EdgeID) -> &mut Edge<NodeID, EdgeID, EdgeData> {
        &mut self.edges[at]
    }
}