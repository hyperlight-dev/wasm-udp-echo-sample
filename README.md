# Wasm UDP Echo Example

## Dependencies

```bash
cargo install cargo-component  # to build the project
cargo install wit-bindgen-cli  # to generate the guest bindings
cargo install wasm-tools       # to get data from the wasm artifacts
cargo install wac              # to link and create the stubs
```

## Run Instructions

```bash
cargo component build
wasmtime run -Sinherit-network target/wasm32-wasip1/debug/udp_echo_server.wasm
```
