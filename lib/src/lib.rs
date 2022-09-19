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
    pub const EDGES: usize = 51;
    pub const NODE_BYTES: usize = 1;
    pub const EDGE_BYTES: usize = 2;
    type NodeData = [u8; NODE_BYTES];
    type EdgeData = [u8; EDGE_BYTES];

    use crate::base::{Node, Edge};
    use crate::collections::GraphList;

    #[test]
    fn create_graph() {
        GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );
    }

    #[test]
    fn test_max_counts() {
        let graph = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        assert_eq!(graph.max_node_count(), NODES);
        assert_eq!(graph.max_edge_count(), EDGES);
    }

    #[test]
    fn init_node_returns_none_when_exceeded() {
        pub const LIMITED_NODES: usize = 5;
        let mut graph: GraphList<LIMITED_NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); LIMITED_NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        let ref0 = graph.init_node(1, [0u8; NODE_BYTES]).unwrap();
        let ref1 = graph.init_node(2, [0u8; NODE_BYTES]).unwrap();
        let ref2 = graph.init_node(3, [0u8; NODE_BYTES]).unwrap();
        let ref3 = graph.init_node(4, [0u8; NODE_BYTES]).unwrap();
        let ref4 = graph.init_node(5, [0u8; NODE_BYTES]).unwrap();
        let ref5 = graph.init_node(6, [0u8; NODE_BYTES]);

        assert_eq!(graph.node(ref0).unwrap().id, 1);
        assert_eq!(graph.node(ref1).unwrap().id, 2);
        assert_eq!(graph.node(ref2).unwrap().id, 3);
        assert_eq!(graph.node(ref3).unwrap().id, 4);
        assert_eq!(graph.node(ref4).unwrap().id, 5);
        assert_eq!(ref5.is_none(), true);
    }

    #[test]
    fn init_edge_returns_none_when_exceeded() {
        pub const LIMITED_EDGES: usize = 5;
        let mut graph: GraphList<NODES, LIMITED_EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); LIMITED_EDGES],
        );


        let node1 = graph.init_node(0, [0u8; NODE_BYTES]).unwrap();
        let node2 = graph.init_node(0, [0u8; NODE_BYTES]).unwrap();

        let ref0 = graph.init_edge(1, node1, node2, [0u8; EDGE_BYTES]).unwrap();
        let ref1 = graph.init_edge(2, node1, node2, [0u8; EDGE_BYTES]).unwrap();
        let ref2 = graph.init_edge(3, node1, node2, [0u8; EDGE_BYTES]).unwrap();
        let ref3 = graph.init_edge(4, node1, node2, [0u8; EDGE_BYTES]).unwrap();
        let ref4 = graph.init_edge(5, node1, node2, [0u8; EDGE_BYTES]).unwrap();
        let ref5 = graph.init_edge(6, node1, node2, [0u8; EDGE_BYTES]);

        assert_eq!(graph.edge(ref0).unwrap().id, 1);
        assert_eq!(graph.edge(ref1).unwrap().id, 2);
        assert_eq!(graph.edge(ref2).unwrap().id, 3);
        assert_eq!(graph.edge(ref3).unwrap().id, 4);
        assert_eq!(graph.edge(ref4).unwrap().id, 5);
        assert_eq!(ref5.is_none(), true);
    }

    #[test]
    fn node_basic() {
        let mut graph: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        let node1 = graph.init_node(123, [5u8; NODE_BYTES]).unwrap();
        let node2 = graph.init_node(345, [7u8; NODE_BYTES]).unwrap();

        assert_eq!(graph.node(node1).unwrap().id, 123);
        assert_eq!(graph.node(node1).unwrap().data, [5u8; NODE_BYTES]);

        assert_eq!(graph.node(node2).unwrap().id, 345);
        assert_eq!(graph.node(node2).unwrap().data, [7u8; NODE_BYTES]);
    }

    #[test]
    fn mut_node_basic() {
        let mut graph: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        let node = graph.init_node(123, [5u8; NODE_BYTES]).unwrap();
        assert_eq!(graph.mut_node(node).unwrap().id, 123);
        assert_eq!(graph.mut_node(node).unwrap().data, [5u8; NODE_BYTES]);

        graph.mut_node(node).unwrap().id = 777;
        graph.mut_node(node).unwrap().data = [7u8; NODE_BYTES];
        assert_eq!(graph.node(node).unwrap().id, 777);
        assert_eq!(graph.node(node).unwrap().data, [7u8; NODE_BYTES]);
        assert_eq!(graph.mut_node(node).unwrap().id, 777);
        assert_eq!(graph.mut_node(node).unwrap().data, [7u8; NODE_BYTES]);
    }

    #[test]
    fn edge_basic() {
        let mut graph: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        let node1 = graph.init_node(123, [0u8; NODE_BYTES]).unwrap();
        let node2 = graph.init_node(345, [0u8; NODE_BYTES]).unwrap();

        let edge1 = graph.init_edge(123, node1, node2, [5u8; EDGE_BYTES]).unwrap();
        let edge2 = graph.init_edge(345, node1, node2, [7u8; EDGE_BYTES]).unwrap();

        assert_eq!(graph.edge(edge1).unwrap().id, 123);
        assert_eq!(graph.edge(edge1).unwrap().data, [5u8; EDGE_BYTES]);

        assert_eq!(graph.edge(edge2).unwrap().id, 345);
        assert_eq!(graph.edge(edge2).unwrap().data, [7u8; EDGE_BYTES]);
    }

    #[test]
    fn mut_edge_basic() {
        let mut graph: GraphList<NODES, EDGES, NodeID, EdgeID, NodeData, EdgeData> = GraphList::new(
            [Node::new(0, [0u8; NODE_BYTES]); NODES],
            [Edge::new(0, [0u8; EDGE_BYTES]); EDGES],
        );

        let node1 = graph.init_node(123, [0u8; NODE_BYTES]).unwrap();
        let node2 = graph.init_node(345, [0u8; NODE_BYTES]).unwrap();

        let edge = graph.init_edge(123, node1, node2, [5u8; EDGE_BYTES]).unwrap();
        assert_eq!(graph.mut_edge(edge).unwrap().id, 123);
        assert_eq!(graph.mut_edge(edge).unwrap().data, [5u8; EDGE_BYTES]);

        graph.mut_edge(edge).unwrap().id = 777;
        graph.mut_edge(edge).unwrap().data = [7u8; EDGE_BYTES];
        assert_eq!(graph.edge(edge).unwrap().id, 777);
        assert_eq!(graph.edge(edge).unwrap().data, [7u8; EDGE_BYTES]);
        assert_eq!(graph.mut_edge(edge).unwrap().id, 777);
        assert_eq!(graph.mut_edge(edge).unwrap().data, [7u8; EDGE_BYTES]);
    }
}