use core::default::Default;
use crate::Edge;

pub struct EdgeList<const EDGES: usize, EdgeID, EdgeData> {
    edges: [Edge<EdgeID, EdgeData>; EDGES]
}

impl<const EDGES: usize, EdgeID, EdgeData> EdgeList<EDGES, EdgeID, EdgeData> {
    pub const fn new(edges: [Edge<EdgeID, EdgeData>; EDGES]) -> Self {
        Self{edges: edges}
    }

    pub fn len(&self) -> usize {
        EDGES
    }

    pub fn get(&self, idx: usize) -> Option<&Edge<EdgeID, EdgeData>> {
        if idx < EDGES {
            Some(&self.edges[idx as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Edge<EdgeID, EdgeData>> {
        if idx < EDGES {
            Some(&mut self.edges[idx as usize])
        } else {
            None
        }
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