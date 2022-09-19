use core::default::Default;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use crate::collections::NodeList;
use crate::collections::EdgeList;
use crate::collections::GraphListNodeRef;
use super::Node;
use super::Edge;
use super::{NodeRef, EdgeRef};

pub struct Graph<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    next_node: AtomicUsize,
    next_edge: AtomicUsize,  
    pub nodes: NodeList<NODES, NodeID, NodeData>,
    pub edges: EdgeList<NODES, EDGES, EdgeID, EdgeData>
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub const fn new(nodes: [Node<NodeID, NodeData>; NODES], edges: [Edge<NODES, EdgeID, EdgeData>; EDGES]) -> Self {
        Self {
            next_node: AtomicUsize::new(0),
            next_edge: AtomicUsize::new(0),
            nodes: NodeList::new(nodes),
            edges: EdgeList::new(edges),
        }
    }

    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> Option<NodeRef> {
        let idx = self.next_node.fetch_add(1, Ordering::AcqRel);
        let node_ref = idx.into();

        match self.mut_node(node_ref) {
            Some(node) => {
                node.id = id;
                node.data = data;
                Some(node_ref)
            },

            None => None
        }
    }

    pub fn init_edge(&mut self, id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) -> Option<EdgeRef> {
        let idx = self.next_edge.fetch_add(1, Ordering::AcqRel);
        let edge_ref = idx.into();

        match self.mut_edge(edge_ref) {
            Some(edge) => {
                edge.id = id;
                edge.a = a;
                edge.b = b;
                edge.data = data;
                Some(edge_ref)
            },

            None => None
        }
    }

    pub fn node(&self, at: NodeRef) -> Option<&Node<NodeID, NodeData>> {
        self.nodes.get(at.idx)
    }

    pub fn mut_node(&mut self, at: NodeRef) -> Option<&mut Node<NodeID, NodeData>> {
        self.nodes.get_mut(at.idx)
    }

    pub fn edge(&self, at: EdgeRef) -> Option<&Edge<NODES, EdgeID, EdgeData>> {
        self.edges.get(at.idx)
    }

    pub fn mut_edge(&mut self, at: EdgeRef) -> Option<&mut Edge<NODES, EdgeID, EdgeData>> {
        self.edges.get_mut(at.idx)
    }
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> Default for Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
    where   NodeID: Default,
            EdgeID: Default,
            NodeData: Default,
            EdgeData: Default
{
    fn default() -> Self {
        Self{
            next_node: AtomicUsize::new(0),
            next_edge: AtomicUsize::new(0),
            nodes: Default::default(),
            edges: Default::default(),
        }
    }
}