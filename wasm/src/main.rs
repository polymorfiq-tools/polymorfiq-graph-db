#![no_std]
#![no_main]
#![feature(const_extern_fn)]

use lib::{Graph, Node, Edge};

extern "C" {}

type NodeID = usize;
type EdgeID = usize;
type NodeDataPointer = usize;
type EdgeDataPointer = usize;
pub const NODES: usize = 5000;
pub const EDGES: usize = 5000;
pub const NODE_DATA_BYTES: usize = 50;
pub const EDGE_DATA_BYTES: usize = 50;
type NodeData = [u8; NODE_DATA_BYTES];
type EdgeData = [u8; EDGE_DATA_BYTES];

static mut GRAPH: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
    next_node: 0,
    next_edge: 0,
    nodes: [Node{id: 0, data: [0u8; NODE_DATA_BYTES], enabled: false}; NODES],
    edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; EDGE_DATA_BYTES], enabled: false}; EDGES],
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