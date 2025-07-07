# Prerequisites
- `cargo install wasm-server-runner`
- `cargo install matchbox_server`

# Running
1. Start matchbox server `matchbox_server`
2. Run the app `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo run cargo run`
    - optionally `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo run cargo watch -x run`
