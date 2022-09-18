#![no_std]
#![no_main]
#![feature(const_extern_fn)]
#![feature(start)]
mod types;
use types::{NodeDataContainer, EdgeDataContainer};
use lib::{Edge, Node, Graph};

// extern crate wee_alloc;
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {}

type NodeID = usize;
type EdgeID = usize;
pub const NODES: usize = 5000;
pub const EDGES: usize = 5000;
pub const NODE_DATA_BYTES: usize = 50;
pub const EDGE_DATA_BYTES: usize = 50;
type NodeData = NodeDataContainer<NODE_DATA_BYTES>;
type EdgeData = EdgeDataContainer<EDGE_DATA_BYTES>;

static mut GRAPH: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph::new(
    Node::new(0, NodeDataContainer::new()),
    Edge::new(0, EdgeDataContainer::new())
);

#[no_mangle]
pub extern "C" fn get_node_count() -> usize {
    unsafe { GRAPH.node_count() }
}

#[no_mangle]
pub extern "C" fn get_edge_count() -> usize {
    unsafe { GRAPH.edge_count() }
}

#[no_mangle]
pub extern "C" fn init_node(id: usize) -> usize {
    unsafe { GRAPH.init_node(id.into(), Default::default()).into()}
}

#[no_mangle]
pub extern "C" fn init_edge(id: usize, a: usize, b: usize) -> usize {
    unsafe { GRAPH.init_edge(id.into(), a.into(), b.into(), Default::default()).into() }
}

#[no_mangle]
pub extern "C" fn node_data(id: usize) -> usize {
    unsafe { GRAPH.node(id.into()).data.as_ptr() as *const () as usize }
}

#[no_mangle]
pub extern "C" fn edge_data(id: usize) -> usize {
    unsafe { GRAPH.edge(id.into()).data.as_ptr() as *const () as usize }
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}