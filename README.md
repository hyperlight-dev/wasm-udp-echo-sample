# Wasm UDP Echo Example

## Prerequisites

1. [Rust](https://www.rust-lang.org/tools/install)
2. [cargo-component](https://github.com/bytecodealliance/cargo-component)
3. [wit-bindgen-cli](https://github.com/bytecodealliance/wit-bindgen)
4. [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
5. [wac](https://github.com/bytecodealliance/wac)

The component tooling dependencies can be installed via `cargo` from a
working Rust installation:

```bash
cargo install cargo-component  # to build the project
cargo install wit-bindgen-cli  # to generate the guest bindings
cargo install wasm-tools       # to get data from the wasm artifacts
cargo install wac              # to link and create the stubs
```

## Building

```bash
cargo component build
```

## Running

To run with
[Hyperlight-Wasm](https://github.com/hyperlight-dev/hyperlight-wasm),
see the [example host
repository](https://github.com/hyperlight-dev/hyperlight-wasm-sockets-example).

To run with wasmtime directly for development and testing:

```bash
wasmtime run -Sinherit-network target/wasm32-wasip1/debug/udp_echo_server.wasm
```

In either case, the wasm component should produce no standard output
and wait forever to receive datagrams via UDP on 127.0.0.1 port 8080.
