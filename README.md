# wasm-info

[![wasm-info](https://github.com/clearloop/wasm-info/workflows/wasm-info/badge.svg)](https://github.com/clearloop/wasm-info)
[![crate](https://img.shields.io/crates/v/wasm-info.svg)](https://crates.io/crates/wasm-info)
[![downloads](https://img.shields.io/crates/d/wasm-info.svg)](https://crates.io/crates/wasm-info)
[![LICENSE](https://img.shields.io/crates/l/wasm-info.svg)](https://choosealicense.com/licenses/MIT/)

This package is a wrapper of the `info` example of [parity-wasm](https://github.com/paritytech/parity-wasm), 
it's really easy to use.

## Example

For example, we write a simple `hello, world` example in `hello_world.rs`.

```rust
//! A hello world example in rust 
#[no_mangle]
pub fn _start() {
    println!("Hello, world!");
}
```

Compile and `wasm-info` it!

```
$ rustc hello_world.rs --target wasm32-unknown-unknown --crate-type=cdylib
$ wasm-info hello_world.wasm
Module sections: 19
  Types: 17
  Functions: 193
  Tables: 1
  Memories: 1
  Globals: 3
  Exports: 4
    memory
    _start
    __data_end
    __heap_base
  Data size: 5945
```


## LICENSE

MIT
