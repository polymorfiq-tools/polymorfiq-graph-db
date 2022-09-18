use crate::{Edge, Node, Graph};
use super::{GraphListNodeRef, GraphListEdgeRef};

#[cfg(not(feature = "alloc"))]
pub struct GraphList<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
}

#[cfg(feature = "alloc")]
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "alloc")]
pub struct GraphList<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    graphs: Vec<Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>>,
    init_nodes: AtomicUsize,
    init_edges: AtomicUsize
}

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(not(feature = "alloc"))]
impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
    where   EdgeID: core::marker::Copy,
            NodeID: core::marker::Copy,
            EdgeData: core::marker::Copy,
            NodeData: core::marker::Copy
{
    pub const fn new(node_default: Node<NodeID, NodeData>, edge_default: Edge<EdgeID, EdgeData>) -> Self {
        Self{graph: Graph::new(node_default, edge_default)}
    }
}

#[cfg(feature = "alloc")]
impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
    where   EdgeID: core::marker::Copy,
            NodeID: core::marker::Copy,
            EdgeData: core::marker::Copy,
            NodeData: core::marker::Copy
{
    pub const fn new(_node_default: Node<NodeID, NodeData>, _edge_default: Edge<EdgeID, EdgeData>) -> Self {
        Self{
            graphs: Vec::new(),
            init_nodes: AtomicUsize::new(0),
            init_edges: AtomicUsize::new(0),
        }
    }
}

#[cfg(not(feature = "alloc"))]
impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub fn max_node_count(&self) -> usize {
        NODES
    }

    pub fn max_edge_count(&self) -> usize {
        EDGES
    }

    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> GraphListNodeRef<NODES> {
        let node_ref = self.graph.init_node(id, data);
        GraphListNodeRef::new(node_ref, 0)
    }

    pub fn init_edge(&mut self, id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) -> GraphListEdgeRef<EDGES> {
        let edge_ref = self.graph.init_edge(id, a.orig, b.orig, data);
        GraphListEdgeRef::new(edge_ref, 0)
    }

    pub fn node_data(&self, list_ref: GraphListNodeRef<NODES>) -> &NodeData {
        &self.graph.node(list_ref.orig).data
    }

    pub fn edge_data(&self, list_ref: GraphListEdgeRef<EDGES>) -> &EdgeData {
        &self.graph.edge(list_ref.orig).data
    }
}

#[cfg(feature = "alloc")]
impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub fn add(&mut self, graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>) {
        self.graphs.push(graph);
    }

    pub fn max_node_count(&self) -> usize {
        NODES * self.graphs.len()
    }

    pub fn max_edge_count(&self) -> usize {
        EDGES * self.graphs.len()
    }

    const fn graph_pos_for_node_idx(idx: usize) -> (usize, usize) {
        (idx / NODES, idx % NODES)
    }

    const fn graph_pos_for_edge_idx(idx: usize) -> (usize, usize) {
        (idx / EDGES, idx % EDGES)
    }

    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> GraphListNodeRef<NODES> {
        let next_node = self.init_nodes.fetch_add(1, Ordering::AcqRel);
        let (graph_num, _) = Self::graph_pos_for_node_idx(next_node);
        let graph = &mut self.graphs[graph_num];
        let node_ref = graph.init_node(id, data);
        GraphListNodeRef::new(node_ref, graph_num)
    }

    pub fn init_edge(&mut self, id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) -> GraphListEdgeRef<EDGES> {
        let next_edge = self.init_edges.fetch_add(1, Ordering::AcqRel);
        let (graph_num, _) = Self::graph_pos_for_edge_idx(next_edge);
        let graph = &mut self.graphs[graph_num];
        let edge_ref = graph.init_edge(id, a.orig, b.orig, data);
        GraphListEdgeRef::new(edge_ref, graph_num)
    }

    pub fn node_data(&self, list_ref: GraphListNodeRef<NODES>) -> &NodeData {
        &self.graphs[list_ref.graph_num].node(list_ref.orig).data
    }

    pub fn edge_data(&self, list_ref: GraphListEdgeRef<EDGES>) -> &EdgeData {
        &self.graphs[list_ref.graph_num].edge(list_ref.orig).data
    }
}