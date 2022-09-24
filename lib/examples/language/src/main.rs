#![no_std]
#![no_main]

extern crate libc;

use lib::{Edge, Node};
use lib::collections::GraphList;

#[derive(Clone, core::marker::Copy)]
enum Relationship {
    None,
    Are,
    Have,
    CanCreate,
    CanRead,
    CanUpdate,
    CanDestroy,
    CanPostTo,
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


    unsafe {
        lib::construct! {GRAPH, Relationship,
            subjects = node "Subjects";
            users = node "Users";
            members = node "Members";
            teams = node "Teams";
            topics = node "Topics";
            posts = node "Posts";
            
            users -Are-> subjects;
            members -Are-> users;
            teams -Have-> members;
            teams -CanPostTo-> topics;
            teams -CanCreate, CanRead, CanUpdate, CanDestroy-> topics;
            topics -Have-> posts;
        };
    }

    /*
     *    - Semantics -
     *    This is defining UNDER WHAT CONDITIONS relationships are implied
     * 
     *    Given a type of object and a fact about it, a relationship may be implied.
     *    The semantics decide what relationships are implied, given facts.
     *    A relationship cannot be implied, that is not part of the Syntax.
     */
    unsafe {
        lib::construct! {GRAPH, Relationship,
            node "Semantics";
        };
    }

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}