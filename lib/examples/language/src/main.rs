#![no_std]
#![no_main]


extern crate libc;

use lib::{Edge, Node};
use lib::collections::GraphList;

#[derive(Clone, core::marker::Copy)]
enum Relationship {
    Is,
    Create,
    Read,
    Update,
    Delete,
    PartOf,
    None
}

type NodeID<'a> = &'a str;
type EdgeID<'a> = Relationship;
pub const NODES: usize = 50;
pub const EDGES: usize = 50;
pub const NODE_DATA_BYTES: usize = 0;
pub const EDGE_DATA_BYTES: usize = 0;
type NodeData = [u8; NODE_DATA_BYTES];
type EdgeData = [u8; EDGE_DATA_BYTES];

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    static mut GRAPH: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
        [Node::new("", [0u8; NODE_DATA_BYTES]); NODES],
        [Edge::new(Relationship::None, [0u8; EDGE_DATA_BYTES]); EDGES]
    );

    let cory = unsafe { GRAPH.init_node("Cory", []).unwrap() };
    let users = unsafe { GRAPH.init_node("Users", []).unwrap() };
    let teams = unsafe { GRAPH.init_node("Teams", []).unwrap() };
    let objects = unsafe { GRAPH.init_node("Objects", []).unwrap() };

    unsafe {
        GRAPH.init_edge(Relationship::Create, users, objects, []);
        GRAPH.init_edge(Relationship::Read, users, objects, []);
        GRAPH.init_edge(Relationship::Update, users, objects, []);
        GRAPH.init_edge(Relationship::Delete, users, objects, []);
        GRAPH.init_edge(Relationship::Is, cory, users, []);
        GRAPH.init_edge(Relationship::PartOf, users, teams, []);
    };

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}