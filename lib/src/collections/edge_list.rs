use core::default::Default;
use core::ops::Index;
use core::ops::IndexMut;
use crate::Edge;

pub struct EdgeList<const EDGES: usize, EdgeID, EdgeData> {
    edges: [Edge<EdgeID, EdgeData>; EDGES]
}

impl<const EDGES: usize, EdgeID, EdgeData> EdgeList<EDGES, EdgeID, EdgeData>
    where   EdgeID: core::marker::Copy,
            EdgeData: core::marker::Copy
{
    pub const fn new(edge: Edge<EdgeID, EdgeData>) -> Self {
        Self{edges: [edge; EDGES]}
    }
}

impl<const EDGES: usize, EdgeID, EdgeData> Index<usize> for EdgeList<EDGES, EdgeID, EdgeData> {
    type Output = Edge<EdgeID, EdgeData>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.edges[idx]
    }
}

impl<const EDGES: usize, EdgeID, EdgeData> IndexMut<usize> for EdgeList<EDGES, EdgeID, EdgeData> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.edges[idx]
    }
}

impl<const EDGES: usize, EdgeID, EdgeData> Default for EdgeList<EDGES, EdgeID, EdgeData>
    where   EdgeID: Default,
            EdgeData: Default
{
    fn default() -> Self {
        Self{edges: [(); EDGES].map(|_| Default::default())}
    }
}