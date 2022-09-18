#![no_std]
#![no_main]
#![feature(const_extern_fn)]
#![feature(start)]
#![feature(default_alloc_error_handler)]
#![feature(int_roundings)]
mod types;
use types::{NodeDataContainer, EdgeDataContainer};
use lib::{Edge, Node};
use lib::collections::GraphList;

#[cfg(feature = "wee_alloc")]
extern crate wee_alloc;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern "C" {
    #[cfg(feature = "alloc")]
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

static mut GRAPHS: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
    Node::new(0, NodeDataContainer::new()),
    Edge::new(0, EdgeDataContainer::new())
);

#[cfg(feature = "alloc")]
#[start]
#[no_mangle]
pub extern "C" fn start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        while (max_node_count() * NODES) < nodes {
            GRAPHS.add(lib::Graph::new(
                Node::new(0, NodeDataContainer::new()),
                Edge::new(0, EdgeDataContainer::new())
            ));
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn max_node_count() -> usize {
    unsafe { GRAPHS.max_node_count() }
}

#[no_mangle]
pub extern "C" fn max_edge_count() -> usize {
    unsafe { GRAPHS.max_edge_count() }
}

#[no_mangle]
pub extern "C" fn init_node(id: usize) -> usize {
    unsafe {
        GRAPHS.init_node(id, Default::default()).into()
    }
}

#[no_mangle]
pub extern "C" fn init_edge(id: usize, a: usize, b: usize) -> usize {
    unsafe {
        GRAPHS.init_edge(id, a.into(), b.into(), Default::default()).into()
    }
}

#[no_mangle]
pub extern "C" fn node_data(ref_idx: usize) -> usize {
    unsafe {
        GRAPHS.node_data(ref_idx.into()).as_ptr() as *const () as usize
    }
}

#[no_mangle]
pub extern "C" fn edge_data(ref_idx: usize) -> usize {
    unsafe {
        GRAPHS.edge_data(ref_idx.into()).as_ptr() as *const () as usize
    }
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}