#![no_std]
#![no_main]
#![feature(const_extern_fn)]
use lib::{Graph, Node, Edge};

#[no_mangle]
const extern "C" fn node_count() -> usize { 5000 }
#[no_mangle]
const extern "C" fn edge_count() -> usize { 5000 }
#[no_mangle]
const extern "C" fn node_data_bytes() -> usize { 5000 }
#[no_mangle]
const extern "C" fn edge_data_bytes() -> usize { 5000 }

type NodeID = usize;
type EdgeID = usize;
type NodeDataPointer = usize;
type EdgeDataPointer = usize;
pub const NODES: usize = node_count();
pub const EDGES: usize = edge_count();
type NodeData = [u8; node_data_bytes()];
type EdgeData = [u8; edge_data_bytes()];

pub static mut GRAPH: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
    next_node: 0,
    next_edge: 0,
    nodes: [Node{id: 0, data: [0u8; node_data_bytes()], enabled: false}; NODES],
    edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; edge_data_bytes()], enabled: false}; EDGES],
};

#[no_mangle]
pub extern "C" fn get_node_count() -> usize {
    unsafe { GRAPH.node_count() }
}

#[no_mangle]
pub extern "C" fn get_edge_count() -> usize {
    unsafe { GRAPH.edge_count() }
}

#[no_mangle]
pub extern "C" fn next_node_id() -> NodeID {
    unsafe { GRAPH.next_node_id() }
}

#[no_mangle]
pub extern "C" fn next_edge_id() -> NodeID {
    unsafe { GRAPH.next_edge_id() }
}

#[no_mangle]
pub extern "C" fn create_enabled_node(id: NodeID) -> NodeDataPointer {
    let mut node = unsafe { GRAPH.find_mutable_node(id) };
    node.enabled = true;
    node.id = id;
    node.data.as_ptr() as *const u8 as NodeDataPointer
}

#[no_mangle]
pub extern "C" fn create_enabled_edge(id: EdgeID, a: NodeID, b: NodeID) -> EdgeDataPointer {
    let mut edge = unsafe { GRAPH.find_mutable_edge(id) };
    edge.enabled = true;
    edge.id = id;
    edge.a = a;
    edge.b = b;
    edge.data.as_ptr() as *const u8 as EdgeDataPointer
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}