use core::default::Default;
use crate::collections::GraphListNodeRef;
use super::NodeRef;

#[derive(Clone, Copy)]
pub struct Edge<const NODES: usize, EdgeID, EdgeData> {
    pub id: EdgeID,
    pub a: GraphListNodeRef<NODES>,
    pub b: GraphListNodeRef<NODES>,
    pub data: EdgeData,
}

impl<const NODES: usize, EdgeID, EdgeData> Edge<NODES, EdgeID, EdgeData> {
    pub const fn new(id: EdgeID, data: EdgeData) -> Self {
        Self{id: id, a: GraphListNodeRef::new(NodeRef::new(0), 0), b: GraphListNodeRef::new(NodeRef::new(0), 0), data: data}
    }
}

impl<const NODES: usize, EdgeID, EdgeData> Default for Edge<NODES, EdgeID, EdgeData>
    where   EdgeID: Default,
            EdgeData: Default
{
    fn default() -> Self {
        Self{
            id: Default::default(),
            a: Default::default(),
            b: Default::default(),
            data: Default::default(),
        }
    }
}