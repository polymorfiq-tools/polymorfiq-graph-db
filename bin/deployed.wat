(module
 (type $none_=>_none (func))
 (type $i32_=>_none (func (param i32)))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (memory $0 16)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 1048576))
 (global $global$2 i32 (i32.const 1048576))
 (export "memory" (memory $0))
 (export "next_edge_id" (func $next_edge_id))
 (export "create_enabled_node" (func $create_enabled_node))
 (export "create_enabled_edge" (func $create_enabled_edge))
 (export "next_node_id" (func $next_edge_id))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $next_edge_id (result i32)
  (call $_ZN3lib4base5graph58Graph$LT$_$C$_$C$NodeID$C$EdgeID$C$NodeData$C$EdgeData$GT$12next_edge_id17hea31ff5903cb594eE)
  (unreachable)
 )
 (func $_ZN3lib4base5graph58Graph$LT$_$C$_$C$NodeID$C$EdgeID$C$NodeData$C$EdgeData$GT$12next_edge_id17hea31ff5903cb594eE
  (call $_ZN4core9panicking5panic17hc2950546692a7448E)
  (unreachable)
 )
 (func $create_enabled_node (param $0 i32) (result i32)
  (call $_ZN3lib4base5graph58Graph$LT$_$C$_$C$NodeID$C$EdgeID$C$NodeData$C$EdgeData$GT$17find_mutable_edge17ha3ee04926da6b0a6E
   (local.get $0)
  )
  (unreachable)
 )
 (func $_ZN3lib4base5graph58Graph$LT$_$C$_$C$NodeID$C$EdgeID$C$NodeData$C$EdgeData$GT$17find_mutable_edge17ha3ee04926da6b0a6E (param $0 i32)
  (call $_ZN4core9panicking18panic_bounds_check17h7a842989aa880d52E
   (local.get $0)
  )
  (unreachable)
 )
 (func $create_enabled_edge (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (call $_ZN3lib4base5graph58Graph$LT$_$C$_$C$NodeID$C$EdgeID$C$NodeData$C$EdgeData$GT$17find_mutable_edge17ha3ee04926da6b0a6E
   (local.get $0)
  )
  (unreachable)
 )
 (func $_ZN4core9panicking9panic_fmt17hbea166bb494aaf18E
  (loop $label$1
   (br $label$1)
  )
 )
 (func $_ZN4core9panicking18panic_bounds_check17h7a842989aa880d52E (param $0 i32)
  (call $_ZN4core9panicking9panic_fmt17hbea166bb494aaf18E)
  (unreachable)
 )
 (func $_ZN4core9panicking5panic17hc2950546692a7448E
  (call $_ZN4core9panicking9panic_fmt17hbea166bb494aaf18E)
  (unreachable)
 )
 ;; custom section ".debug_info", size 134
 ;; custom section ".debug_pubtypes", size 18
 ;; custom section ".debug_ranges", size 32
 ;; custom section ".debug_abbrev", size 80
 ;; custom section ".debug_line", size 129
 ;; custom section ".debug_str", size 362
 ;; custom section ".debug_pubnames", size 88
 ;; custom section "producers", size 75
)
