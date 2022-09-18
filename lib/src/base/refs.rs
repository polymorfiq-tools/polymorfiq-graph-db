#[derive(Clone, Copy)]
pub struct NodeRef { pub(crate) idx: usize }
impl NodeRef { pub(crate) const fn new(idx: usize) -> Self { Self{idx: idx} } }
impl Into<usize> for NodeRef { fn into(self) -> usize { self.idx } }
impl Into<NodeRef> for usize { fn into(self) -> NodeRef { NodeRef{idx: self} } }
impl Default for NodeRef { fn default() -> Self { NodeRef{idx: 0} } }

#[derive(Clone, Copy)]
pub struct EdgeRef { pub(crate) idx: usize }
impl EdgeRef { pub(crate) const fn new(idx: usize) -> Self { Self{idx: idx} } }
impl Into<usize> for EdgeRef { fn into(self) -> usize { self.idx } }
impl Into<EdgeRef> for usize { fn into(self) -> EdgeRef { EdgeRef{idx: self} } }
impl Default for EdgeRef { fn default() -> Self { EdgeRef{idx: 0} } }