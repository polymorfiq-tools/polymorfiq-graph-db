(module
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_i32_i32_=>_i32 (func (param i32 i32 i32) (result i32)))
 (type $none_=>_i32 (func (result i32)))
 (memory $0 7647)
 (table $0 1 1 funcref)
 (global $__stack_pointer (mut i32) (i32.const 1048576))
 (global $global$1 i32 (i32.const 501128584))
 (global $global$2 i32 (i32.const 501128592))
 (export "memory" (memory $0))
 (export "max_edge_count" (func $max_edge_count))
 (export "init_node" (func $init_node))
 (export "init_edge" (func $init_edge))
 (export "node_data" (func $node_data))
 (export "edge_data" (func $edge_data))
 (export "max_node_count" (func $max_edge_count))
 (export "__data_end" (global $global$1))
 (export "__heap_base" (global $global$2))
 (func $max_edge_count (result i32)
  (i32.const 5000)
 )
 (func $init_node (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (global.set $__stack_pointer
   (local.tee $1
    (i32.sub
     (global.get $__stack_pointer)
     (i32.const 150000)
    )
   )
  )
  (local.set $2
   (i32.const 0)
  )
  (loop $label$1
   (i32.store align=1
    (local.tee $3
     (i32.add
      (i32.add
       (local.get $1)
       (i32.const 100000)
      )
      (local.get $2)
     )
    )
    (i32.const 0)
   )
   (i32.store8
    (i32.add
     (local.get $3)
     (i32.const 4)
    )
    (i32.const 0)
   )
   (br_if $label$1
    (i32.ne
     (local.tee $2
      (i32.add
       (local.get $2)
       (i32.const 5)
      )
     )
     (i32.const 50000)
    )
   )
  )
  (i32.store16 offset=99997 align=1
   (local.get $1)
   (i32.load16_u offset=100000 align=1
    (local.get $1)
   )
  )
  (i32.store8 offset=99999
   (local.get $1)
   (i32.load8_u
    (i32.add
     (i32.add
      (local.get $1)
      (i32.const 100000)
     )
     (i32.const 2)
    )
   )
  )
  (local.set $4
   (i32.load offset=100003 align=1
    (local.get $1)
   )
  )
  (local.set $5
   (i32.load offset=100007 align=1
    (local.get $1)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $1)
     (i32.const 49996)
    )
    (i32.add
     (local.get $1)
     (i32.const 100011)
    )
    (i32.const 49989)
   )
  )
  (i32.store8
   (local.tee $2
    (i32.add
     (i32.add
      (local.get $1)
      (i32.const 49992)
     )
     (i32.const 2)
    )
   )
   (i32.load8_u offset=99999
    (local.get $1)
   )
  )
  (i32.store16 offset=49992
   (local.get $1)
   (i32.load16_u offset=99997 align=1
    (local.get $1)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $1)
     (i32.const 3)
    )
    (i32.add
     (local.get $1)
     (i32.const 49996)
    )
    (i32.const 49989)
   )
  )
  (i32.store8
   (local.tee $6
    (i32.add
     (i32.add
      (local.get $1)
      (i32.const 49996)
     )
     (i32.const 2)
    )
   )
   (i32.load8_u
    (local.get $2)
   )
  )
  (i32.store16 offset=49996
   (local.get $1)
   (i32.load16_u offset=49992
    (local.get $1)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $1)
     (i32.const 100000)
    )
    (i32.add
     (local.get $1)
     (i32.const 3)
    )
    (i32.const 49989)
   )
  )
  (local.set $3
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
  (block $label$2
   (br_if $label$2
    (i32.gt_u
     (local.get $2)
     (i32.const 4999)
    )
   )
   (i32.store
    (i32.add
     (local.tee $3
      (i32.mul
       (local.get $2)
       (i32.const 50004)
      )
     )
     (i32.const 1048584)
    )
    (local.get $0)
   )
   (i32.store16 align=1
    (i32.add
     (local.get $3)
     (i32.const 1048588)
    )
    (i32.load16_u offset=49996
     (local.get $1)
    )
   )
   (i32.store8
    (i32.add
     (local.get $3)
     (i32.const 1048590)
    )
    (i32.load8_u
     (local.get $6)
    )
   )
   (i32.store align=1
    (i32.add
     (local.get $3)
     (i32.const 1048595)
    )
    (local.get $5)
   )
   (i32.store align=1
    (i32.add
     (local.get $3)
     (i32.const 1048591)
    )
    (local.get $4)
   )
   (drop
    (call $memcpy
     (i32.add
      (local.get $3)
      (i32.const 1048599)
     )
     (i32.add
      (local.get $1)
      (i32.const 100000)
     )
     (i32.const 49989)
    )
   )
   (local.set $3
    (local.get $2)
   )
  )
  (global.set $__stack_pointer
   (i32.add
    (local.get $1)
    (i32.const 150000)
   )
  )
  (local.get $3)
 )
 (func $init_edge (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (global.set $__stack_pointer
   (local.tee $3
    (i32.sub
     (global.get $__stack_pointer)
     (i32.const 150000)
    )
   )
  )
  (local.set $4
   (i32.rem_u
    (local.get $2)
    (i32.const 5000)
   )
  )
  (local.set $5
   (i32.rem_u
    (local.get $1)
    (i32.const 5000)
   )
  )
  (local.set $2
   (i32.const 0)
  )
  (loop $label$1
   (i32.store align=1
    (local.tee $1
     (i32.add
      (i32.add
       (local.get $3)
       (i32.const 100000)
      )
      (local.get $2)
     )
    )
    (i32.const 0)
   )
   (i32.store8
    (i32.add
     (local.get $1)
     (i32.const 4)
    )
    (i32.const 0)
   )
   (br_if $label$1
    (i32.ne
     (local.tee $2
      (i32.add
       (local.get $2)
       (i32.const 5)
      )
     )
     (i32.const 50000)
    )
   )
  )
  (i32.store16 offset=99997 align=1
   (local.get $3)
   (i32.load16_u offset=100000 align=1
    (local.get $3)
   )
  )
  (i32.store8 offset=99999
   (local.get $3)
   (i32.load8_u
    (i32.add
     (i32.add
      (local.get $3)
      (i32.const 100000)
     )
     (i32.const 2)
    )
   )
  )
  (local.set $6
   (i32.load offset=100003 align=1
    (local.get $3)
   )
  )
  (local.set $7
   (i32.load offset=100007 align=1
    (local.get $3)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $3)
     (i32.const 49996)
    )
    (i32.add
     (local.get $3)
     (i32.const 100011)
    )
    (i32.const 49989)
   )
  )
  (i32.store8
   (local.tee $2
    (i32.add
     (i32.add
      (local.get $3)
      (i32.const 49992)
     )
     (i32.const 2)
    )
   )
   (i32.load8_u offset=99999
    (local.get $3)
   )
  )
  (i32.store16 offset=49992
   (local.get $3)
   (i32.load16_u offset=99997 align=1
    (local.get $3)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $3)
     (i32.const 3)
    )
    (i32.add
     (local.get $3)
     (i32.const 49996)
    )
    (i32.const 49989)
   )
  )
  (i32.store8
   (local.tee $8
    (i32.add
     (i32.add
      (local.get $3)
      (i32.const 49996)
     )
     (i32.const 2)
    )
   )
   (i32.load8_u
    (local.get $2)
   )
  )
  (i32.store16 offset=49996
   (local.get $3)
   (i32.load16_u offset=49992
    (local.get $3)
   )
  )
  (drop
   (call $memcpy
    (i32.add
     (local.get $3)
     (i32.const 100000)
    )
    (i32.add
     (local.get $3)
     (i32.const 3)
    )
    (i32.const 49989)
   )
  )
  (local.set $2
   (i32.const 0)
  )
  (i32.store offset=1048580
   (i32.const 0)
   (i32.add
    (local.tee $1
     (i32.load offset=1048580
      (i32.const 0)
     )
    )
    (i32.const 1)
   )
  )
  (block $label$2
   (br_if $label$2
    (i32.gt_u
     (local.get $1)
     (i32.const 4999)
    )
   )
   (i32.store
    (i32.add
     (local.tee $2
      (i32.mul
       (local.get $1)
       (i32.const 50012)
      )
     )
     (i32.const 251068592)
    )
    (local.get $4)
   )
   (i32.store
    (i32.add
     (local.get $2)
     (i32.const 251068588)
    )
    (local.get $5)
   )
   (i32.store
    (i32.add
     (local.get $2)
     (i32.const 251068584)
    )
    (local.get $0)
   )
   (i32.store16 align=1
    (i32.add
     (local.get $2)
     (i32.const 251068596)
    )
    (i32.load16_u offset=49996
     (local.get $3)
    )
   )
   (i32.store8
    (i32.add
     (local.get $2)
     (i32.const 251068598)
    )
    (i32.load8_u
     (local.get $8)
    )
   )
   (i32.store align=1
    (i32.add
     (local.get $2)
     (i32.const 251068603)
    )
    (local.get $7)
   )
   (i32.store align=1
    (i32.add
     (local.get $2)
     (i32.const 251068599)
    )
    (local.get $6)
   )
   (drop
    (call $memcpy
     (i32.add
      (local.get $2)
      (i32.const 251068607)
     )
     (i32.add
      (local.get $3)
      (i32.const 100000)
     )
     (i32.const 49989)
    )
   )
   (local.set $2
    (local.get $1)
   )
  )
  (global.set $__stack_pointer
   (i32.add
    (local.get $3)
    (i32.const 150000)
   )
  )
  (local.get $2)
 )
 (func $node_data (param $0 i32) (result i32)
  (i32.add
   (i32.mul
    (i32.rem_u
     (local.get $0)
     (i32.const 5000)
    )
    (i32.const 50004)
   )
   (i32.const 1048588)
  )
 )
 (func $edge_data (param $0 i32) (result i32)
  (i32.add
   (i32.mul
    (i32.rem_u
     (local.get $0)
     (i32.const 5000)
    )
    (i32.const 50012)
   )
   (i32.const 251068596)
  )
 )
 (func $memcpy (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (call $_ZN17compiler_builtins3mem6memcpy17h92fde4e84ee29bc6E
   (local.get $0)
   (local.get $1)
   (local.get $2)
  )
 )
 (func $_ZN17compiler_builtins3mem6memcpy17h92fde4e84ee29bc6E (param $0 i32) (param $1 i32) (param $2 i32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  (local $8 i32)
  (local $9 i32)
  (local $10 i32)
  (block $label$1
   (block $label$2
    (br_if $label$2
     (i32.gt_u
      (local.get $2)
      (i32.const 15)
     )
    )
    (local.set $3
     (local.get $0)
    )
    (br $label$1)
   )
   (local.set $5
    (i32.add
     (local.get $0)
     (local.tee $4
      (i32.and
       (i32.sub
        (i32.const 0)
        (local.get $0)
       )
       (i32.const 3)
      )
     )
    )
   )
   (block $label$3
    (br_if $label$3
     (i32.eqz
      (local.get $4)
     )
    )
    (local.set $3
     (local.get $0)
    )
    (local.set $6
     (local.get $1)
    )
    (loop $label$4
     (i32.store8
      (local.get $3)
      (i32.load8_u
       (local.get $6)
      )
     )
     (local.set $6
      (i32.add
       (local.get $6)
       (i32.const 1)
      )
     )
     (br_if $label$4
      (i32.lt_u
       (local.tee $3
        (i32.add
         (local.get $3)
         (i32.const 1)
        )
       )
       (local.get $5)
      )
     )
    )
   )
   (local.set $3
    (i32.add
     (local.get $5)
     (local.tee $8
      (i32.and
       (local.tee $7
        (i32.sub
         (local.get $2)
         (local.get $4)
        )
       )
       (i32.const -4)
      )
     )
    )
   )
   (block $label$5
    (block $label$6
     (br_if $label$6
      (i32.eqz
       (local.tee $6
        (i32.and
         (local.tee $9
          (i32.add
           (local.get $1)
           (local.get $4)
          )
         )
         (i32.const 3)
        )
       )
      )
     )
     (br_if $label$5
      (i32.lt_s
       (local.get $8)
       (i32.const 1)
      )
     )
     (local.set $1
      (i32.add
       (local.tee $10
        (i32.and
         (local.get $9)
         (i32.const -4)
        )
       )
       (i32.const 4)
      )
     )
     (local.set $4
      (i32.and
       (i32.sub
        (i32.const 0)
        (local.tee $2
         (i32.shl
          (local.get $6)
          (i32.const 3)
         )
        )
       )
       (i32.const 24)
      )
     )
     (local.set $6
      (i32.load
       (local.get $10)
      )
     )
     (loop $label$7
      (i32.store
       (local.get $5)
       (i32.or
        (i32.shr_u
         (local.get $6)
         (local.get $2)
        )
        (i32.shl
         (local.tee $6
          (i32.load
           (local.get $1)
          )
         )
         (local.get $4)
        )
       )
      )
      (local.set $1
       (i32.add
        (local.get $1)
        (i32.const 4)
       )
      )
      (br_if $label$7
       (i32.lt_u
        (local.tee $5
         (i32.add
          (local.get $5)
          (i32.const 4)
         )
        )
        (local.get $3)
       )
      )
      (br $label$5)
     )
    )
    (br_if $label$5
     (i32.lt_s
      (local.get $8)
      (i32.const 1)
     )
    )
    (local.set $1
     (local.get $9)
    )
    (loop $label$8
     (i32.store
      (local.get $5)
      (i32.load
       (local.get $1)
      )
     )
     (local.set $1
      (i32.add
       (local.get $1)
       (i32.const 4)
      )
     )
     (br_if $label$8
      (i32.lt_u
       (local.tee $5
        (i32.add
         (local.get $5)
         (i32.const 4)
        )
       )
       (local.get $3)
      )
     )
    )
   )
   (local.set $2
    (i32.and
     (local.get $7)
     (i32.const 3)
    )
   )
   (local.set $1
    (i32.add
     (local.get $9)
     (local.get $8)
    )
   )
  )
  (block $label$9
   (br_if $label$9
    (i32.eqz
     (local.get $2)
    )
   )
   (local.set $5
    (i32.add
     (local.get $3)
     (local.get $2)
    )
   )
   (loop $label$10
    (i32.store8
     (local.get $3)
     (i32.load8_u
      (local.get $1)
     )
    )
    (local.set $1
     (i32.add
      (local.get $1)
      (i32.const 1)
     )
    )
    (br_if $label$10
     (i32.lt_u
      (local.tee $3
       (i32.add
        (local.get $3)
        (i32.const 1)
       )
      )
      (local.get $5)
     )
    )
   )
  )
  (local.get $0)
 )
 ;; custom section ".debug_info", size 4693
 ;; custom section ".debug_pubtypes", size 36
 ;; custom section ".debug_ranges", size 1192
 ;; custom section ".debug_abbrev", size 249
 ;; custom section ".debug_line", size 2677
 ;; custom section ".debug_str", size 6697
 ;; custom section ".debug_pubnames", size 1804
 ;; custom section "producers", size 75
)
