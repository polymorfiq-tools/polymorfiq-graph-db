use core::default::Default;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;
use super::Node;
use super::Edge;
use super::{NodeRef, EdgeRef};
use crate::collections::NodeList;
use crate::collections::EdgeList;

pub struct Graph<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> {
    next_node: AtomicUsize,
    next_edge: AtomicUsize,  
    pub nodes: NodeList<NODES, NodeID, NodeData>,
    pub edges: EdgeList<EDGES, EdgeID, EdgeData>
}

impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> {
    pub fn init_node(&mut self, id: NodeID, data: NodeData) -> NodeRef {
        let idx = self.next_node.fetch_add(1, Ordering::AcqRel);
        let node_ref = idx.into();

        let mut node = self.mut_node(node_ref);
        node.id = id;
        node.data = data;

        node_ref
    }

    pub fn init_edge(&mut self, id: EdgeID, a: NodeRef, b: NodeRef, data: EdgeData) -> EdgeRef {
        let idx = self.next_edge.fetch_add(1, Ordering::AcqRel);
        let edge_ref = idx.into();

        let mut edge = self.mut_edge(edge_ref);
        edge.id = id;
        edge.a = a;
        edge.b = b;
        edge.data = data;

        edge_ref
    }

    pub fn node(&self, at: NodeRef) -> &Node<NodeID, NodeData> {
        &self.nodes[at.idx]
    }

    pub fn mut_node(&mut self, at: NodeRef) -> &mut Node<NodeID, NodeData> {
        &mut self.nodes[at.idx]
    }

    pub fn edge(&self, at: EdgeRef) -> &Edge<EdgeID, EdgeData> {
        &self.edges[at.idx]
    }

    pub fn mut_edge(&mut self, at: EdgeRef) -> &mut Edge<EdgeID, EdgeData> {
        &mut self.edges[at.idx]
    }
}


impl<const NODES: usize, const EDGES: usize, NodeID, EdgeID, NodeData, EdgeData> Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>
    where   EdgeID: core::marker::Copy,
            NodeID: core::marker::Copy,
            NodeData: core::marker::Copy,
            EdgeData: core::marker::Copy
{
    pub const fn new(default_node: Node<NodeID, NodeData>, default_edge: Edge<EdgeID, EdgeData>) -> Self {
        Self {
            next_node: AtomicUsize::new(0),
            next_edge: AtomicUsize::new(0),
            nodes: NodeList::new(default_node),
            edges: EdgeList::new(default_edge),
        }
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