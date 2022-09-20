#![no_std]
#![no_main]


extern crate libc;

use lib::{Edge, Node};
use lib::collections::GraphList;
use lib::collections::GraphListNodeRef;

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

static mut GRAPH: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
    [Node::new("", [0u8; NODE_DATA_BYTES]); NODES],
    [Edge::new(Relationship::None, [0u8; EDGE_DATA_BYTES]); EDGES]
);

fn init_node(id: NodeID<'static>, data: NodeData) -> Option<GraphListNodeRef<NODES>> {
    unsafe { GRAPH.init_node(id, data) }
}

fn init_edge(id: EdgeID, a: GraphListNodeRef<NODES>, b: GraphListNodeRef<NODES>, data: EdgeData) {
    unsafe { GRAPH.init_edge(id, a, b, data) };
}

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let cory = init_node("Cory", []).unwrap();
    let users = init_node("Users", []).unwrap();
    let teams = init_node("Teams", []).unwrap();
    let objects = init_node("Objects", []).unwrap();

    init_edge(Relationship::Create, users, objects, []);
    init_edge(Relationship::Read, users, objects, []);
    init_edge(Relationship::Update, users, objects, []);
    init_edge(Relationship::Delete, users, objects, []);
    init_edge(Relationship::Is, cory, users, []);
    init_edge(Relationship::PartOf, users, teams, []);

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}