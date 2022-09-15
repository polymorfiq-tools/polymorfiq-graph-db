(module
 (type $none_=>_i32 (func (result i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (memory $0 781)
 (global $global$0 i32 (i32.const 51168584))
 (global $global$1 i32 (i32.const 51168592))
 (export "memory" (memory $0))
 (export "edge_count" (func $0))
 (export "next_node_id" (func $1))
 (export "next_edge_id" (func $2))
 (export "create_enabled_node" (func $3))
 (export "create_enabled_edge" (func $4))
 (export "node_count" (func $0))
 (export "node_data_bytes" (func $0))
 (export "edge_data_bytes" (func $0))
 (export "get_node_count" (func $0))
 (export "get_edge_count" (func $0))
 (export "__data_end" (global $global$0))
 (export "__heap_base" (global $global$1))
 (func $0 (result i32)
  (i32.const 5000)
 )
 (func $1 (result i32)
  (local $0 i32)
  (i32.store
   (i32.const 1048576)
   (local.tee $0
    (i32.rem_u
     (i32.add
      (i32.load
       (i32.const 1048576)
      )
      (i32.const 1)
     )
     (i32.const 5000)
    )
   )
  )
  (local.get $0)
 )
 (func $2 (result i32)
  (local $0 i32)
  (i32.store
   (i32.const 1048580)
   (local.tee $0
    (i32.rem_u
     (i32.add
      (i32.load
       (i32.const 1048580)
      )
      (i32.const 1)
     )
     (i32.const 5000)
    )
   )
  )
  (local.get $0)
 )
 (func $3 (param $0 i32) (result i32)
  (local $1 i32)
  (if
   (i32.ge_u
    (local.get $0)
    (i32.const 5000)
   )
   (loop $label$2
    (br $label$2)
   )
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
 (func $4 (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (if
   (i32.ge_u
    (local.get $0)
    (i32.const 5000)
   )
   (loop $label$2
    (br $label$2)
   )
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
 ;; custom section "producers", size 75
)
