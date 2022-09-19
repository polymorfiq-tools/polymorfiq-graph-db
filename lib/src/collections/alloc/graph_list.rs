extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::{Edge, Node, Graph};
use crate::collections::{GraphListNodeRef, GraphListEdgeRef};

pub struct GraphList<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    graphs: Vec<Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>>,
    init_nodes: AtomicUsize,
    init_edges: AtomicUsize,
    initial_graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>,
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub const fn new(nodes: [Node<NodeID, NodeData>; NODES], edges: [Edge<NODES, EdgeID, EdgeData>; EDGES]) -> Self {
        Self{
            graphs: Vec::new(),
            init_nodes: AtomicUsize::new(0),
            init_edges: AtomicUsize::new(0),
            initial_graph: Graph::new(nodes, edges),
        }
    }

    pub fn add(&mut self, graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>) {
        self.graphs.push(graph);
    }

    pub fn max_node_count(&self) -> usize {
        NODES * self.len()
    }

    pub fn max_edge_count(&self) -> usize {
        EDGES * self.len()
    }

    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> Option<GraphListNodeRef<NODES>> {
        let next_node = self.init_nodes.fetch_add(1, Ordering::AcqRel);
        let (graph_num, _) = Self::graph_pos_for_node_idx(next_node);

        if graph_num < self.len() {
            match self.mut_graph(graph_num).init_node(id, data) {
                Some(node_ref) => Some(GraphListNodeRef::new(node_ref, graph_num)),
                None => None
            }
        } else {
            None
        }
    }

    pub fn init_edge(&mut self, id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) -> Option<GraphListEdgeRef<EDGES>> {
        let next_edge = self.init_edges.fetch_add(1, Ordering::AcqRel);
        let (graph_num, _) = Self::graph_pos_for_edge_idx(next_edge);

        if graph_num < self.len() {
            match self.mut_graph(graph_num).init_edge(id, a, b, data) {
                Some(edge_ref) => Some(GraphListEdgeRef::new(edge_ref, graph_num)),
                None => None
            }
        } else {
            None
        }
    }

    pub fn node(&self, list_ref: GraphListNodeRef<NODES>) -> Option<&Node<NodeID, NodeData>> {
        if list_ref.graph_num < self.len() {
            match self.graph(list_ref.graph_num).node(list_ref.orig) {
                Some(node) => Some(node),
                None => None
            }
        } else {
            None
        }
    }

    pub fn mut_node(&mut self, list_ref: GraphListNodeRef<NODES>) -> Option<&mut Node<NodeID, NodeData>> {
        if list_ref.graph_num < self.len() {
            match self.mut_graph(list_ref.graph_num).mut_node(list_ref.orig) {
                Some(node) => Some(node),
                None => None
            }
        } else {
            None
        }
    }

    pub fn edge(&self, list_ref: GraphListEdgeRef<EDGES>) -> Option<&Edge<NODES, EdgeID, EdgeData>> {
        if list_ref.graph_num < self.len() {
            match self.graph(list_ref.graph_num).edge(list_ref.orig) {
                Some(edge) => Some(edge),
                None => None
            }
        } else {
            None
        }
    }

    pub fn mut_edge(&mut self, list_ref: GraphListEdgeRef<EDGES>) -> Option<&mut Edge<NODES, EdgeID, EdgeData>> {
        if list_ref.graph_num < self.len() {
            match self.mut_graph(list_ref.graph_num).mut_edge(list_ref.orig) {
                Some(edge) => Some(edge),
                None => None
            }
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.graphs.len() + 1
    }

    fn mut_graph(&mut self, idx: usize) -> &mut Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
        if idx == 0 {
            &mut self.initial_graph
        } else {
            &mut self.graphs[idx-1]
        }
    }

    fn graph(&self, idx: usize) -> &Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
        if idx == 0 {
            &self.initial_graph
        } else {
            &self.graphs[idx-1]
        }
    }

    const fn graph_pos_for_node_idx(idx: usize) -> (usize, usize) {
        (idx / NODES, idx % NODES)
    }

    const fn graph_pos_for_edge_idx(idx: usize) -> (usize, usize) {
        (idx / EDGES, idx % EDGES)
    }
}