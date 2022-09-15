(module
 (type $none_=>_i32 (func (result i32)))
 (type $none_=>_none (func))
 (type $i32_=>_none (func (param i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (memory $0 781)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 51168584))
 (global $global$2 i32 (i32.const 51168592))
 (export "memory" (memory $0))
 (export "edge_count" (func $edge_count))
 (export "next_node_id" (func $next_node_id))
 (export "next_edge_id" (func $next_edge_id))
 (export "create_enabled_node" (func $create_enabled_node))
 (export "create_enabled_edge" (func $create_enabled_edge))
 (export "node_count" (func $edge_count))
 (export "node_data_bytes" (func $edge_count))
 (export "edge_data_bytes" (func $edge_count))
 (export "get_node_count" (func $edge_count))
 (export "get_edge_count" (func $edge_count))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $edge_count (result i32)
  (i32.const 5000)
 )
 (func $next_node_id (result i32)
  (local $0 i32)
  (i32.store offset=1048576
   (i32.const 0)
   (local.tee $0
    (i32.rem_u
     (i32.add
      (i32.load offset=1048576
       (i32.const 0)
      )
      (i32.const 1)
     )
     (i32.const 5000)
    )
   )
  )
  (local.get $0)
 )
 (func $next_edge_id (result i32)
  (local $0 i32)
  (i32.store offset=1048580
   (i32.const 0)
   (local.tee $0
    (i32.rem_u
     (i32.add
      (i32.load offset=1048580
       (i32.const 0)
      )
      (i32.const 1)
     )
     (i32.const 5000)
    )
   )
  )
  (local.get $0)
 )
 (func $create_enabled_node (param $0 i32) (result i32)
  (local $1 i32)
  (block $label$1
   (br_if $label$1
    (i32.lt_u
     (local.get $0)
     (i32.const 5000)
    )
   )
   (call $_ZN4core9panicking18panic_bounds_check17h7a842989aa880d52E
    (local.get $0)
   )
   (unreachable)
  )
  (i32.store
   (i32.add
    (local.tee $1
     (i32.mul
      (local.get $0)
      (i32.const 5008)
     )
    )
    (i32.const 1048584)
   )
   (local.get $0)
  )
  (i32.store8
   (i32.add
    (local.get $1)
    (i32.const 1053588)
   )
   (i32.const 1)
  )
  (i32.add
   (local.get $1)
   (i32.const 1048588)
  )
 )
 (func $_ZN4core9panicking18panic_bounds_check17h7a842989aa880d52E (param $0 i32)
  (call $_ZN4core9panicking9panic_fmt17hbea166bb494aaf18E)
  (unreachable)
 )
 (func $create_enabled_edge (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (block $label$1
   (br_if $label$1
    (i32.lt_u
     (local.get $0)
     (i32.const 5000)
    )
   )
   (call $_ZN4core9panicking18panic_bounds_check17h7a842989aa880d52E
    (local.get $0)
   )
   (unreachable)
  )
  (i32.store
   (i32.add
    (local.tee $3
     (i32.mul
      (local.get $0)
      (i32.const 5016)
     )
    )
    (i32.const 26088592)
   )
   (local.get $2)
  )
  (i32.store
   (i32.add
    (local.get $3)
    (i32.const 26088588)
   )
   (local.get $1)
  )
  (i32.store
   (i32.add
    (local.get $3)
    (i32.const 26088584)
   )
   (local.get $0)
  )
  (i32.store8
   (i32.add
    (local.get $3)
    (i32.const 26093596)
   )
   (i32.const 1)
  )
  (i32.add
   (local.get $3)
   (i32.const 26088596)
  )
 )
 (func $_ZN4core9panicking9panic_fmt17hbea166bb494aaf18E
  (loop $label$1
   (br $label$1)
  )
 )
 ;; custom section ".debug_info", size 106
 ;; custom section ".debug_pubtypes", size 18
 ;; custom section ".debug_ranges", size 24
 ;; custom section ".debug_abbrev", size 80
 ;; custom section ".debug_line", size 110
 ;; custom section ".debug_str", size 311
 ;; custom section ".debug_pubnames", size 78
 ;; custom section "producers", size 75
)
