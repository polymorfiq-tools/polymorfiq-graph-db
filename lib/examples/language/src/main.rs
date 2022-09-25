#![no_std]
#![no_main]
#![allow(dead_code)]

extern crate libc;

use lib::{Edge, Node};
use lib::collections::GraphList;

#[derive(Clone, core::marker::Copy)]
enum Syntax {
    None,
    Is,
    Has,
    Owns,
    Reads
}

pub const SYNTAX_NODES: usize = 50;
pub const SYNTAX_EDGES: usize = 50;
static mut SYNTAX: GraphList<SYNTAX_NODES, SYNTAX_EDGES, &str, Syntax, [u8; 0], [u8; 0]> = {
    GraphList::new(
        [Node::new("", [0u8; 0]); SYNTAX_NODES],
        [Edge::new(Syntax::None, [0u8; 0]); SYNTAX_EDGES]
    )
};

#[derive(Clone, core::marker::Copy)]
enum Action {
    ReplyTo,
}

#[derive(Clone, core::marker::Copy)]
enum Semantics {
    None,
    ReflectsOn,
    Relates(Syntax),
    Allowed(Action)
}

#[derive(Clone, Copy)]
enum SemanticID<'a> {
    None,
    Rule(&'a str),
    RuleStart(&'a str, usize),
    RuleEnd(&'a str, usize),
    Subject(&'a str, usize),
    Object(&'a str),
}

pub const SEMANTIC_NODES: usize = 50;
pub const SEMANTIC_EDGES: usize = 50;
static mut SEMANTICS: GraphList<SEMANTIC_NODES, SEMANTIC_EDGES, SemanticID, Semantics, [u8; 0], [u8; 0]> = {
    GraphList::new(
        [Node::new(SemanticID::None, [0u8; 0]); SEMANTIC_NODES],
        [Edge::new(Semantics::None, [0u8; 0]); SEMANTIC_EDGES]
    )
};

#[no_mangle]
pub extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
    /*
     *    - Syntax -
     *    Syntax defines what explicit relationship are possible in the graph.
     * 
     *    It states the concepts that exist in the universe.
     *    It states the relationships that exist in the universe
     *    Anything not stated here, is through implication or otherwise derived.
     */
    unsafe {
        lib::construct! {SYNTAX, Syntax,
            subjects = node "Subjects";
            users = node "Users";
            members = node "Members";
            teams = node "Teams";
            topics = node "Topics";
            posts = node "Posts";
            
            users -Is-> subjects;
            members -Has-> users;
            teams -Has-> members;
            teams -Owns-> topics;
            teams -Reads-> topics;
            topics -Has-> posts;
        };
    }

    /*
     *    - Semantics -
     *    Semantics define what is IMPLIED by a given state of the universe.
     * 
     *    Given a set of subjects, semantic rules can check their relationship
     *    If the state of the universe in the check is observed, the implication is true as well
     *    These implications cannot be explicitly written into the universe - they are derived
     */
    unsafe {
        lib::construct! {SEMANTICS, Semantics,
            user_can_post = node SemanticID::Rule("user_can_post");
            
            user = node SemanticID::Subject("Users", 0);
            post = node SemanticID::Subject("Post", 0);
            members = node SemanticID::Object("Members");
            team = node SemanticID::Object("Teams");
            topic = node SemanticID::Object("Topics");

            user <-Relates(Syntax::Has)- members;
            members <-Relates(Syntax::Has)- team;
            team -Relates(Syntax::Owns)-> topic;
            topic -Relates(Syntax::Has)-> post;

            user_can_post -ReflectsOn-> user;
            user -Allowed(Action::ReplyTo)-> post;
        };
    }

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}