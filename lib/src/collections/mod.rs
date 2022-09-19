mod node_list;
pub use node_list::NodeList;

mod edge_list;
pub use edge_list::EdgeList;

#[cfg(feature = "alloc")]
mod alloc;

#[cfg(feature = "alloc")]
pub use alloc::GraphList;


#[cfg(not(feature = "alloc"))]
mod graph_list;

#[cfg(not(feature = "alloc"))]
pub use graph_list::GraphList;

mod refs;
pub use refs::GraphListNodeRef;
pub use refs::GraphListEdgeRef;