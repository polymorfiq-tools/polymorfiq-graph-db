use crate::{Edge, Node, Graph};
use super::{GraphListNodeRef, GraphListEdgeRef};

pub struct GraphList<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub const fn new(nodes: [Node<NodeID, NodeData>; NODES], edges: [Edge<NODES, EdgeID, EdgeData>; EDGES]) -> Self {
        Self{graph: Graph::new(nodes, edges)}
    }
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub fn max_node_count(&self) -> usize {
        NODES
    }

    pub fn max_edge_count(&self) -> usize {
        EDGES
    }

    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> Option<GraphListNodeRef<NODES>> {
        match self.graph.init_node(id, data) {
            Some(node_ref) => Some(GraphListNodeRef::new(node_ref, 0)),
            None => None
        }
    }

    pub fn init_edge(&mut self, id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) -> Option<GraphListEdgeRef<EDGES>> {
        match self.graph.init_edge(id, a, b, data) {
            Some(edge_ref) => Some(GraphListEdgeRef::new(edge_ref, 0)),
            None => None
        }
    }

    pub fn node(&self, list_ref: GraphListNodeRef<NODES>) -> Option<&Node<NodeID, NodeData>> {
        match self.graph.node(list_ref.orig) {
            Some(node) => Some(node),
            None => None
        }
    }

    pub fn mut_node(&mut self, list_ref: GraphListNodeRef<NODES>) -> Option<&mut Node<NodeID, NodeData>> {
        match self.graph.mut_node(list_ref.orig) {
            Some(node) => Some(node),
            None => None
        }
    }

    pub fn edge(&self, list_ref: GraphListEdgeRef<EDGES>) -> Option<&Edge<NODES, EdgeID, EdgeData>> {
        match self.graph.edge(list_ref.orig) {
            Some(edge) => Some(edge),
            None => None
        }
    }

    pub fn mut_edge(&mut self, list_ref: GraphListEdgeRef<EDGES>) -> Option<&mut Edge<NODES, EdgeID, EdgeData>> {
        match self.graph.mut_edge(list_ref.orig) {
            Some(edge) => Some(edge),
            None => None
        }
    }
}