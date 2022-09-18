#![no_std]
mod base;
pub use base::Node;
pub use base::Edge;
pub use base::Graph;
pub use base::NodeRef;
pub use base::EdgeRef;

pub mod collections;

#[cfg(test)]
mod tests {
    type NodeID = usize;
    type EdgeID = usize;
    pub const NODES: usize = 50;
    pub const EDGES: usize = 50;
    pub const NODE_BYTES: usize = 0;
    pub const EDGE_BYTES: usize = 0;
    type NodeData = [u8; NODE_BYTES];
    type EdgeData = [u8; EDGE_BYTES];

    use crate::base::{Node, Edge, Graph};

    #[test]
    fn create_graph() {
        let mut _graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; EDGES],
        };
    }

    #[test]
    fn next_node_id_cycles_nodes() {
        pub const LIMITED_NODES: usize = 5;
        let mut graph: Graph<LIMITED_NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; LIMITED_NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; EDGES],
        };

        assert_eq!(graph.next_node_id(), 1);
        assert_eq!(graph.next_node_id(), 2);
        assert_eq!(graph.next_node_id(), 3);
        assert_eq!(graph.next_node_id(), 4);
        assert_eq!(graph.next_node_id(), 0);
    }

    #[test]
    fn nodes_start_disabled() {
        pub const LIMITED_NODES: usize = 3;
        let graph: Graph<LIMITED_NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; LIMITED_NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; EDGES],
        };

        assert_eq!(graph.find_node(0).enabled, false);
        assert_eq!(graph.find_node(1).enabled, false);
        assert_eq!(graph.find_node(2).enabled, false);
    }

    #[test]
    fn edges_start_disabled() {
        pub const LIMITED_EDGES: usize = 3;
        let graph: Graph<NODES, LIMITED_EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; LIMITED_EDGES],
        };

        assert_eq!(graph.find_edge(0).enabled, false);
        assert_eq!(graph.find_edge(1).enabled, false);
        assert_eq!(graph.find_edge(2).enabled, false);
    }

    #[test]
    fn next_edge_id_cycles_nodes() {
        pub const LIMITED_EDGES: usize = 5;
        let mut graph: Graph<NODES, LIMITED_EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; LIMITED_EDGES],
        };

        assert_eq!(graph.next_edge_id(), 1);
        assert_eq!(graph.next_edge_id(), 2);
        assert_eq!(graph.next_edge_id(), 3);
        assert_eq!(graph.next_edge_id(), 4);
        assert_eq!(graph.next_edge_id(), 0);
    }

    #[test]
    fn find_mutable_node_basic() {
        let mut graph: Graph<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = Graph {
            next_node: 0,
            next_edge: 0,
            nodes: [Node{id: 0, data: [0u8; EDGE_BYTES], enabled: false}; NODES],
            edges: [Edge{id: 0, a: 0, b: 0, data: [0u8; NODE_BYTES], enabled: false}; EDGES],
        };

        graph.find_mutable_node(20).id = 5;
        graph.find_mutable_node(20).enabled = true;
        assert_eq!(graph.nodes[20].id, 5);
        assert_eq!(graph.nodes[20].enabled, true);

        graph.find_mutable_node(14).id = 5;
        graph.find_mutable_node(14).enabled = true;
        assert_eq!(graph.nodes[14].id, 5);
        assert_eq!(graph.nodes[14].enabled, true);
    }
}