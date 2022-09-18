#![no_std]
#![no_main]
#![feature(const_extern_fn)]
#![feature(start)]
#![feature(default_alloc_error_handler)]
#![feature(int_roundings)]
mod types;
use types::{NodeDataContainer, EdgeDataContainer};
use lib::{Edge, Node, Graph};

#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

extern "C" {
    static nodes: usize;
}

type NodeID = usize;
type EdgeID = usize;
pub const NODES: usize = 5000;
pub const EDGES: usize = 5000;
pub const NODE_DATA_BYTES: usize = 50000;
pub const EDGE_DATA_BYTES: usize = 50000;
type NodeData = NodeDataContainer<NODE_DATA_BYTES>;
type EdgeData = EdgeDataContainer<EDGE_DATA_BYTES>;

#[cfg(not(feature = "alloc"))]
static mut GRAPHS: [Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>; 1] = [Graph::new(
    Node::new(0, NodeDataContainer::new()),
    Edge::new(0, EdgeDataContainer::new())
); 1];

#[cfg(feature = "alloc")]
static mut GRAPHS: Vec<Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData>> = Vec::new();

#[cfg(feature = "alloc")]
#[start]
#[no_mangle]
pub extern "C" fn start(_: isize, _: *const *const u8) -> isize {

    let mut i = 0;

    unsafe {
        while (i * NODES) < nodes {
                GRAPHS.push(Graph::new(
                    Node::new(0, NodeDataContainer::new()),
                    Edge::new(0, EdgeDataContainer::new())
                ));

            i += 1;
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn get_node_count() -> usize {
    unsafe { GRAPHS.len() * NODES }
}

#[no_mangle]
pub extern "C" fn get_edge_count() -> usize {
    unsafe { GRAPHS.len() * NODES }
}

#[no_mangle]
pub extern "C" fn init_node(id: usize) -> usize {
    unsafe { GRAPHS[id.div_floor(NODES)].init_node(id.into(), Default::default()).into()}
}

#[no_mangle]
pub extern "C" fn init_edge(id: usize, a: usize, b: usize) -> usize {
    unsafe { GRAPHS[id.div_floor(EDGES)].init_edge(id.into(), a.into(), b.into(), Default::default()).into() }
}

#[no_mangle]
pub extern "C" fn node_data(id: usize) -> usize {
    unsafe { GRAPHS[id.div_floor(NODES)].node(id.into()).data.as_ptr() as *const () as usize }
}

#[no_mangle]
pub extern "C" fn edge_data(id: usize) -> usize {
    unsafe { GRAPHS[id.div_floor(EDGES)].edge(id.into()).data.as_ptr() as *const () as usize }
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}