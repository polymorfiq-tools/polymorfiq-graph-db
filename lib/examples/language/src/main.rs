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
    MemberOf,
    None
}

enum Fact {
    Is,
    MemberOf
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
    /*
     *    - Syntax -
     *    This is defining what is possible in the graph.
     * 
     *    It states the concepts that exist in the universe.
     *    It states the relationships that ARE POSSIBLE to make.
     *    If a relationship between two concepts is not stated here, it cannot happen.
     */

    let syntax = init_node("Syntax", []).unwrap();
    let users = init_node("Users", []).unwrap();
    init_edge(Relationship::PartOf, users, syntax, []);
    
    let teams = init_node("Teams", []).unwrap();
    init_edge(Relationship::PartOf, teams, syntax, []);
    init_edge(Relationship::MemberOf, users, teams, []);

    let objects = init_node("Objects", []).unwrap();
    init_edge(Relationship::PartOf, objects, syntax, []);
    init_edge(Relationship::Create, teams, objects, []);
    init_edge(Relationship::Read, teams, objects, []);
    init_edge(Relationship::Update, teams, objects, []);
    init_edge(Relationship::Delete, teams, objects, []);

    /*
     *    - Semantics -
     *    This is defining UNDER WHAT CONDITIONS relationships are implied
     * 
     *    Given a type of object and a fact about it, a relationship may be implied.
     *    The semantics decide what relationships are implied, given facts.
     *    A relationship cannot be implied, that is not part of the Syntax.
     */
    let _semantics = init_node("Semantics", []).unwrap();

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}