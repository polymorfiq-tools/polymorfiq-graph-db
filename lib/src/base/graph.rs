use super::Node;
use super::Edge;

pub struct Graph<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    next_node: NodeID,
    next_edge: EdgeID,
    nodes: [Node<NodeID, NodeData>; NODES],
    edges: [Edge<NodeID, EdgeID, EdgeData>; EDGES]
}

impl<const NODES: usize, const EDGES: usize, NodeID: core::ops::Add<usize> + core::slice::SliceIndex<Edge>, EdgeID: core::ops::Add<usize> + core::slice::SliceIndex<Edge>, NodeData, EdgeData> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub const fn new(init_node_id: NodeID, init_node_data: NodeData, init_edge_id: EdgeID, init_edge_data: EdgeData) -> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
        where   Node<NodeID, NodeData>: core::marker::Copy,
                Edge<NodeID, EdgeID, EdgeData>: core::marker::Copy,
    {
        Graph{
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: init_node_data}; NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: init_edge_data}; EDGES],
        }
    }

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