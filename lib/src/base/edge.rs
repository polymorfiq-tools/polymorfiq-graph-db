use core::default::Default;
use super::NodeRef;

#[derive(Clone, Copy)]
pub struct Edge<EdgeID, EdgeData> {
    pub id: EdgeID,
    pub a: NodeRef,
    pub b: NodeRef,
    pub data: EdgeData,
}

impl<EdgeID, EdgeData> Edge<EdgeID, EdgeData> {
    pub const fn new(id: EdgeID, data: EdgeData) -> Self {
        Self{id: id, a: NodeRef::new(0), b: NodeRef::new(0), data: data}
    }
}

impl<EdgeID, EdgeData> Default for Edge<EdgeID, EdgeData>
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