use core::default::Default;
use crate::Edge;

pub struct EdgeList<const NODES: usize, const EDGES: usize, EdgeID, EdgeData> {
    edges: [Edge<NODES, EdgeID, EdgeData>; EDGES]
}

impl<const NODES: usize, const EDGES: usize, EdgeID, EdgeData> EdgeList<NODES, EDGES, EdgeID, EdgeData> {
    pub const fn new(edges: [Edge<NODES, EdgeID, EdgeData>; EDGES]) -> Self {
        Self{edges: edges}
    }

    pub fn len(&self) -> usize {
        EDGES
    }

    pub fn get(&self, idx: usize) -> Option<&Edge<NODES, EdgeID, EdgeData>> {
        if idx < EDGES {
            Some(&self.edges[idx as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Edge<NODES, EdgeID, EdgeData>> {
        if idx < EDGES {
            Some(&mut self.edges[idx as usize])
        } else {
            None
        }
    }
}

impl<const NODES: usize, const EDGES: usize, EdgeID, EdgeData> Default for EdgeList<NODES, EDGES, EdgeID, EdgeData>
    where   EdgeID: Default,
            EdgeData: Default
{
    fn default() -> Self {
        Self{edges: [(); EDGES].map(|_| Default::default())}
    }
}