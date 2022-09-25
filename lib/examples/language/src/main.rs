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
    CanCreate,
    CanRead,
    CanUpdate,
    CanDestroy,
    CanPostTo,
    CanReplyTo,
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
enum Semantics {
    None,
    StartsAt,
    ReflectsOn,
    Relates(Syntax),
    Implied(Syntax)
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
     *    This is defining what is possible in the graph.
     * 
     *    It states the concepts that exist in the universe.
     *    It states the relationships that ARE POSSIBLE to make.
     *    If a relationship between two concepts is not stated here, it cannot happen.
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
            teams -CanPostTo-> topics;
            teams -CanCreate, CanRead, CanUpdate, CanDestroy-> topics;
            topics -Has-> posts;
            users -CanReplyTo-> posts;
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
        lib::construct! {SEMANTICS, Semantics,
            user_can_post = node SemanticID::Rule("user_can_post");
            
            user = node SemanticID::Subject("Users", 0);
            post = node SemanticID::Subject("Post", 0);
            members = node SemanticID::Object("Members");
            team = node SemanticID::Object("Teams");
            topic = node SemanticID::Object("Topics");

            user_can_post -StartsAt-> user;

            user <-Relates(Syntax::Has)- members;
            members <-Relates(Syntax::Has)- team;
            team -Relates(Syntax::CanPostTo)-> topic;
            topic -Relates(Syntax::Has)-> post;

            user_can_post -ReflectsOn-> user;
            user -Implied(Syntax::CanReplyTo)-> post;
        };
    }

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}