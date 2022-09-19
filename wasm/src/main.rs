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

const fn parse_u32(s: &str)->usize {
    let mut out:u32 = 0;
    let mut i:usize = 0;
    while i<s.len() {
        out *= 10;
        out += (s.as_bytes()[i] - b'0') as u32;
        i += 1;
    }
    out as usize
}

type NodeID = usize;
type EdgeID = usize;
pub const NODES: usize = parse_u32(env!("NODES_PER_GRAPH", "NODES_PER_GRAPH not set"));
pub const EDGES: usize = parse_u32(env!("EDGES_PER_GRAPH", "EDGES_PER_GRAPH not set"));
pub const NODE_DATA_BYTES: usize = parse_u32(env!("NODE_DATA_BYTES", "NODE_DATA_BYTES not set"));
pub const EDGE_DATA_BYTES: usize = parse_u32(env!("EDGE_DATA_BYTES", "EDGE_DATA_BYTES not set"));
type NodeData = NodeDataContainer<NODE_DATA_BYTES>;
type EdgeData = EdgeDataContainer<EDGE_DATA_BYTES>;

static mut GRAPHS: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
    [Node::new(0, NodeDataContainer::new()); NODES],
    [Edge::new(0, EdgeDataContainer::new()); EDGES]
);

#[cfg(feature = "alloc")]
#[start]
#[no_mangle]
pub extern "C" fn start(_: isize, _: *const *const u8) -> isize {
    unsafe {
        while (max_node_count() * NODES) < nodes {
            GRAPHS.add(lib::Graph::new(
                [Node::new(0, NodeDataContainer::new()); NODES],
                [Edge::new(0, EdgeDataContainer::new()); EDGES]
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
pub extern "C" fn node_data_bytes() -> usize {
    NODE_DATA_BYTES
}

#[no_mangle]
pub extern "C" fn edge_data_bytes() -> usize {
    EDGE_DATA_BYTES
}

#[no_mangle]
pub extern "C" fn init_node(id: usize) -> usize {
    match unsafe { GRAPHS.init_node(id, Default::default()) } {
        Some(node) => node.into(),
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn init_edge(id: usize, a: usize, b: usize) -> usize {
    match unsafe { GRAPHS.init_edge(id, a.into(), b.into(), Default::default()) } {
        Some(edge) => edge.into(),
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn node_id(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.node(ref_idx.into()) } {
        Some(node) => node.id as *const () as usize,
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn set_node_id(ref_idx: usize, id: usize) -> usize {
    match unsafe { GRAPHS.mut_node(ref_idx.into()) } {
        Some(node) => {
            node.id = id.into();
            1
        },
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn node_data_ptr(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.node(ref_idx.into()) } {
        Some(node) => node.data.as_ptr() as *const () as usize,
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn edge_id(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.edge(ref_idx.into()) } {
        Some(edge) => edge.id as *const () as usize,
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn edge_a(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.edge(ref_idx.into()) } {
        Some(edge) => edge.a.into(),
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn edge_b(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.edge(ref_idx.into()) } {
        Some(edge) => edge.b.into(),
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn edge_data_ptr(ref_idx: usize) -> usize {
    match unsafe { GRAPHS.edge(ref_idx.into()) } {
        Some(edge) => edge.data.as_ptr() as *const () as usize,
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn set_edge_id(ref_idx: usize, id: usize) -> usize {
    match unsafe { GRAPHS.mut_edge(ref_idx.into()) } {
        Some(edge) => {
            edge.id = id.into();
            1
        },
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn set_edge_a(ref_idx: usize, a: usize) -> usize {
    match unsafe { GRAPHS.mut_edge(ref_idx.into()) } {
        Some(edge) => {
            edge.a = a.into();
            1
        },
        None => 0
    }
}

#[no_mangle]
pub extern "C" fn set_edge_b(ref_idx: usize, b: usize) -> usize {
    match unsafe { GRAPHS.mut_edge(ref_idx.into()) } {
        Some(edge) => {
            edge.b = b.into();
            1
        },
        None => 0
    }
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}