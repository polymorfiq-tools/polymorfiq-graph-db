(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $none_=>_i32 (func (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (memory $0 21)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 1328584))
 (global $global$2 i32 (i32.const 1328592))
 (export "memory" (memory $0))
 (export "max_node_count" (func $max_node_count))
 (export "max_edge_count" (func $max_edge_count))
 (export "edge_data_bytes" (func $edge_data_bytes))
 (export "init_node" (func $init_node))
 (export "init_edge" (func $init_edge))
 (export "node_id" (func $node_id))
 (export "set_node_id" (func $set_node_id))
 (export "node_data_ptr" (func $node_data_ptr))
 (export "edge_id" (func $edge_id))
 (export "edge_a" (func $edge_a))
 (export "edge_b" (func $edge_b))
 (export "edge_data_ptr" (func $edge_data_ptr))
 (export "set_edge_id" (func $set_edge_id))
 (export "set_edge_a" (func $set_edge_a))
 (export "set_edge_b" (func $set_edge_b))
 (export "node_data_bytes" (func $edge_data_bytes))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $max_node_count (result i32)
  (i32.const 5000)
 )
 (func $max_edge_count (result i32)
  (i32.const 10000)
 )
 (func $edge_data_bytes (result i32)
  (i32.const 4)
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
     (i32.const 4999)
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
  (local $5 i32)
  (local $6 i32)
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
     (i32.const 9999)
    )
   )
   (i32.store
    (i32.add
     (local.tee $3
      (i32.mul
       (local.get $4)
       (i32.const 24)
      )
     )
     (i32.const 1088604)
    )
    (i32.const 0)
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1088600)
    )
    (local.tee $5
     (i32.div_u
      (local.get $2)
      (i32.const 5000)
     )
    )
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1088592)
    )
    (local.tee $6
     (i32.div_u
      (local.get $1)
      (i32.const 5000)
     )
    )
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1088584)
    )
    (local.get $0)
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1088596)
    )
    (i32.add
     (i32.mul
      (local.get $5)
      (i32.const -5000)
     )
     (local.get $2)
    )
   )
   (i32.store
    (i32.add
     (local.get $3)
     (i32.const 1088588)
    )
    (i32.add
     (i32.mul
      (local.get $6)
      (i32.const -5000)
     )
     (local.get $1)
    )
   )
   (local.set $3
    (local.get $4)
   )
  )
  (local.get $3)
 )
 (func $node_id (param $0 i32) (result i32)
  (i32.load
   (i32.add
    (i32.shl
     (i32.rem_u
      (local.get $0)
      (i32.const 5000)
     )
     (i32.const 3)
    )
    (i32.const 1048584)
   )
  )
 )
 (func $set_node_id (param $0 i32) (param $1 i32) (result i32)
  (i32.store
   (i32.add
    (i32.shl
     (i32.rem_u
      (local.get $0)
      (i32.const 5000)
     )
     (i32.const 3)
    )
    (i32.const 1048584)
   )
   (local.get $1)
  )
  (i32.const 1)
 )
 (func $node_data_ptr (param $0 i32) (result i32)
  (i32.add
   (i32.shl
    (i32.rem_u
     (local.get $0)
     (i32.const 5000)
    )
    (i32.const 3)
   )
   (i32.const 1048588)
  )
 )
 (func $edge_id (param $0 i32) (result i32)
  (i32.load
   (i32.add
    (i32.mul
     (i32.rem_u
      (local.get $0)
      (i32.const 10000)
     )
     (i32.const 24)
    )
    (i32.const 1088584)
   )
  )
 )
 (func $edge_a (param $0 i32) (result i32)
  (i32.add
   (i32.mul
    (i32.load
     (i32.add
      (local.tee $0
       (i32.mul
        (i32.rem_u
         (local.get $0)
         (i32.const 10000)
        )
        (i32.const 24)
       )
      )
      (i32.const 1088592)
     )
    )
    (i32.const 5000)
   )
   (i32.load
    (i32.add
     (local.get $0)
     (i32.const 1088588)
    )
   )
  )
 )
 (func $edge_b (param $0 i32) (result i32)
  (i32.add
   (i32.mul
    (i32.load
     (i32.add
      (local.tee $0
       (i32.mul
        (i32.rem_u
         (local.get $0)
         (i32.const 10000)
        )
        (i32.const 24)
       )
      )
      (i32.const 1088600)
     )
    )
    (i32.const 5000)
   )
   (i32.load
    (i32.add
     (local.get $0)
     (i32.const 1088596)
    )
   )
  )
 )
 (func $edge_data_ptr (param $0 i32) (result i32)
  (i32.add
   (i32.mul
    (i32.rem_u
     (local.get $0)
     (i32.const 10000)
    )
    (i32.const 24)
   )
   (i32.const 1088604)
  )
 )
 (func $set_edge_id (param $0 i32) (param $1 i32) (result i32)
  (i32.store
   (i32.add
    (i32.mul
     (i32.rem_u
      (local.get $0)
      (i32.const 10000)
     )
     (i32.const 24)
    )
    (i32.const 1088584)
   )
   (local.get $1)
  )
  (i32.const 1)
 )
 (func $set_edge_a (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (i32.store
   (i32.add
    (local.tee $0
     (i32.mul
      (i32.rem_u
       (local.get $0)
       (i32.const 10000)
      )
      (i32.const 24)
     )
    )
    (i32.const 1088592)
   )
   (local.tee $2
    (i32.div_u
     (local.get $1)
     (i32.const 5000)
    )
   )
  )
  (i32.store
   (i32.add
    (local.get $0)
    (i32.const 1088588)
   )
   (i32.add
    (i32.mul
     (local.get $2)
     (i32.const -5000)
    )
    (local.get $1)
   )
  )
  (i32.const 1)
 )
 (func $set_edge_b (param $0 i32) (param $1 i32) (result i32)
  (local $2 i32)
  (i32.store
   (i32.add
    (local.tee $0
     (i32.mul
      (i32.rem_u
       (local.get $0)
       (i32.const 10000)
      )
      (i32.const 24)
     )
    )
    (i32.const 1088600)
   )
   (local.tee $2
    (i32.div_u
     (local.get $1)
     (i32.const 5000)
    )
   )
  )
  (i32.store
   (i32.add
    (local.get $0)
    (i32.const 1088596)
   )
   (i32.add
    (i32.mul
     (local.get $2)
     (i32.const -5000)
    )
    (local.get $1)
   )
  )
  (i32.const 1)
 )
 ;; custom section "producers", size 75
)
