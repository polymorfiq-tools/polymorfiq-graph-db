#[macro_export]
macro_rules! construct {
    () => {};
    ($graph:expr $(,)*) => {};
    ($graph:expr, $rel:ident $(,)*) => {};

    ($graph:expr, $rel:ident, node $nodeID:expr $(;)*) => {
        $graph.init_node($nodeID, Default::default()).unwrap();
    };

    ($graph:expr, $rel:ident, node $nodeID:expr, $nodeData:expr $(;)*) => {
        $graph.init_node($nodeID, $nodeData).unwrap();
    };

    ($graph:expr, $rel:ident, node $($e:expr)+; $($tt:tt)*) => {
        $crate::construct! {$graph, $rel, node $($e)+}
        $crate::construct! {$graph, $rel, $($tt)*}
    };

    ($graph:expr, $rel:ident, $var:ident = node $($e:expr)+ $(;)*) => {
        let $var = $crate::construct! {$graph, $rel, node $($e)+};
    };

    ($graph:expr, $rel:ident, $var:ident = node $($e:expr)+; $($tt:tt)*) => {
        $crate::construct! {$graph, $rel, $var = node $($e)+};
        $crate::construct! {$graph, $rel, $($tt)*};
    };

    ($graph:expr, $rel:ident, $nodeA:ident -$($edge:ident$(($val:expr))*$(.data($data:expr))*),+-> $nodeB:ident $(;)*) => {
        $(
            let mut data = Default::default();
            $(data = $data)*;
            $graph.init_edge($rel::$edge$(($val))*, $nodeA, $nodeB, data).unwrap();
        )+
    };

    ($graph:expr, $rel:ident, $nodeA:ident -$($edge:ident$(($val:expr))*$(.data($data:expr))*),+-> $nodeB:ident; $($tt:tt)*) => {
        $crate::construct! {$graph, $rel, $nodeA -$($edge$(($val))*$(.data($data))*),+-> $nodeB}
        $crate::construct! {$graph, $rel, $($tt)*}
    };

    ($graph:expr, $rel:ident, $nodeB:ident <-$($edge:ident$(($val:expr))*$(.data($data:expr))*),+- $nodeA:ident $(;)*) => {
        $(
            let mut data = Default::default();
            $(data = $data)*;
            $graph.init_edge($rel::$edge$(($val))*, $nodeA, $nodeB, data).unwrap();
        )+
    };

    ($graph:expr, $rel:ident, $nodeB:ident <-$($edge:ident$(($val:expr))*$(.data($data:expr))*),+- $nodeA:ident; $($tt:tt)*) => {
        $crate::construct! {$graph, $rel, $nodeA <-$($edge$(($val))*$(.data($data))*),+- $nodeB}
        $crate::construct! {$graph, $rel, $($tt)*}
    };
}