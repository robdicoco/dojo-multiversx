(module $factorial.wasm
  (type (;0;) (func (param i32) (result i32)))
  (func $factorial (type 0) (param i32) (result i32)
    (local i64)
    block  ;; label = @1
      local.get 0
      br_if 0 (;@1;)
      i32.const 1
      return
    end
    i32.const 0
    local.get 0
    i64.extend_i32_u
    local.get 0
    i32.const -1
    i32.add
    call $factorial
    i64.extend_i32_u
    i64.mul
    local.tee 1
    i32.wrap_i64
    local.get 1
    i64.const 32
    i64.shr_u
    i32.wrap_i64
    select)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 16)
  (global $__stack_pointer (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (global (;2;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "factorial" (func $factorial))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2)))
