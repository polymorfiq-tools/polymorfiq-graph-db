#![no_std]
#![no_main]
use lib::{Graph, Node, Edge};

pub const NODES: usize = 50;
pub const EDGES: usize = 50;
type NodeID = usize;
type EdgeID = usize;
type NodeData = [u8; 10];
type EdgeData = [u8; 10];

pub static mut GRAPH: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
    next_node: 0,
    next_edge: 0,
    nodes: [Node{id: 0, data: [0u8; 10], enabled: false}; NODES],
    edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; 10], enabled: false}; EDGES],
};

extern "C" {

}

#[no_mangle]
pub extern "C" fn next_node() -> usize {
    unsafe { GRAPH.next_node() as *const Node<NodeID, NodeData> as usize }
}

// Panic Handling
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}