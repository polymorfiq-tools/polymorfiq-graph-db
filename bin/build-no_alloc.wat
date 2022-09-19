(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (memory $0 47)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 3048584))
 (global $global$2 i32 (i32.const 3048592))
 (export "memory" (memory $0))
 (export "max_node_count" (func $max_node_count))
 (export "max_edge_count" (func $max_edge_count))
 (export "init_node" (func $init_node))
 (export "init_edge" (func $init_edge))
 (export "node_data" (func $node_data))
 (export "edge_data" (func $edge_data))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $max_node_count (result i32)
  (i32.const 50000)
 )
 (func $max_edge_count (result i32)
  (i32.const 100000)
 )
 (func $init_node (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local.set $1
   (i32.const 0)
  )
  (i32.store offset=1048576
   (i32.const 0)
   (i32.add
    (local.tee $2
     (i32.load offset=1048576
      (i32.const 0)
     )
    )
    (i32.const 1)
   )
  )
  (block $label$1
   (br_if $label$1
    (i32.gt_u
     (local.get $2)
     (i32.const 49999)
    )
   )
   (i32.store
    (i32.add
     (local.tee $1
      (i32.shl
       (local.get $2)
       (i32.const 3)
      )
     )
     (i32.const 1048588)
    )
    (i32.const 0)
   )
   (i32.store
    (i32.add
     (local.get $1)
     (i32.const 1048584)
    )
    (local.get $0)
   )
   (local.set $1
    (local.get $2)
   )
  )
  (local.get $1)
 )
 (func $init_edge (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local.set $3
   (i32.const 0)
  )
  (i32.store offset=1048580
   (i32.const 0)
   (i32.add
    (local.tee $4
     (i32.load offset=1048580
      (i32.const 0)
     )
    )
    (i32.const 1)
   )
  )
  (block $label$1
   (br_if $label$1
    (i32.gt_u
     (local.get $4)
     (i32.const 99999)
    )
   )
   (i32.store
    (i32.add
     (local.tee $3
      (i32.shl
       (local.get $4)
       (i32.const 4)
      )
     )
     (i32.const 1448596)
    )
    (i32.const 0)
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1448592)
    )
    (i32.rem_u
     (local.get $2)
     (i32.const 50000)
    )
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1448588)
    )
    (i32.rem_u
     (local.get $1)
     (i32.const 50000)
    )
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1448584)
    )
    (local.get $0)
   )
   (local.set $3
    (local.get $4)
   )
  )
  (local.get $3)
 )
 (func $node_data (param $0 i32) (result i32)
  (i32.add
   (i32.shl
    (i32.rem_u
     (local.get $0)
     (i32.const 50000)
    )
    (i32.const 3)
   )
   (i32.const 1048588)
  )
 )
 (func $edge_data (param $0 i32) (result i32)
  (i32.add
   (i32.shl
    (i32.rem_u
     (local.get $0)
     (i32.const 100000)
    )
    (i32.const 4)
   )
   (i32.const 1448596)
  )
 )
 ;; custom section "producers", size 75
)
