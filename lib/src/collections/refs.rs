use crate::{NodeRef, EdgeRef};

#[derive(Clone, Copy)]
pub struct GraphListNodeRef<const NODES: usize>{ pub(super) orig: NodeRef, pub(super) graph_num: usize }
impl<const NODES: usize> GraphListNodeRef<NODES> {
    pub(super) const fn new(node_ref: NodeRef, graph_num: usize) -> Self {
        Self{orig: node_ref, graph_num: graph_num}
    }
}
impl<const NODES: usize> Into<usize> for GraphListNodeRef<NODES> {
    fn into(self) -> usize {
        let rel_idx: usize = self.orig.into();
        let abs_idx = (NODES * self.graph_num) + rel_idx;
        abs_idx
    }
}
impl<const NODES: usize> Into<GraphListNodeRef<NODES>> for usize {
    fn into(self) -> GraphListNodeRef<NODES> {
        let graph_num = self / NODES;
        let rel_idx = self % NODES;

        GraphListNodeRef{
            orig: NodeRef::new(rel_idx),
            graph_num: graph_num
        }
    }
}
impl<const NODES: usize> Default for GraphListNodeRef<NODES> {
    fn default() -> Self { GraphListNodeRef{orig: Default::default(), graph_num: 0} }
}

#[derive(Clone, Copy)]
pub struct GraphListEdgeRef<const EDGES: usize>{ pub(super) orig: EdgeRef, pub(super) graph_num: usize }
impl<const EDGES: usize> GraphListEdgeRef<EDGES> {
    pub(super) const fn new(edge_ref: EdgeRef, graph_num: usize) -> Self {
        Self{orig: edge_ref, graph_num: graph_num}
    }
}
impl<const EDGES: usize> Into<usize> for GraphListEdgeRef<EDGES> {
    fn into(self) -> usize {
        let rel_idx: usize = self.orig.into();
        let abs_idx = (EDGES * self.graph_num) + rel_idx;
        abs_idx
    }
}
impl<const EDGES: usize> Into<GraphListEdgeRef<EDGES>> for usize {
    fn into(self) -> GraphListEdgeRef<EDGES> {
        let graph_num = self / EDGES;
        let rel_idx = self % EDGES;

        GraphListEdgeRef{
            orig: EdgeRef::new(rel_idx),
            graph_num: graph_num
        }
    }
}
impl<const EDGES: usize> Default for GraphListEdgeRef<EDGES> {
    fn default() -> Self { GraphListEdgeRef{orig: Default::default(), graph_num: 0} }
}