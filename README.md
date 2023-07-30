# wasm-examples
Example wasm programs used for debugging host functions setup, CI testing, etc.



## Rust Examples

### Print Hello World

the example requires rustup, rustc, and cargo to be installed.

Building:

```bash
cargo build --target wasm32-wasi --release
```

Testing (with wasmtime-cli):
```bash
 wasmtime target/wasm32-wasi/release/print-hello-world.wasm
```