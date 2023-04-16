# Setup

```shell
cargo install wasm-tools
rustup target add wasm32-wasi
```

# Guest


```shell
cargo build --target wasm32-wasi --release
```

```shell
ls -lh target/wasm32-wasi/{debug,release}/guest.wasm
-rwxr-xr-x  1 amitu  staff   2.1M Apr 16 21:59 target/wasm32-wasi/debug/guest.wasm
-rwxr-xr-x  1 amitu  staff    14K Apr 16 22:00 target/wasm32-wasi/release/guest.wasm
```

```shell
wasm-tools component new ./target/wasm32-wasi/release/guest.wasm -o my-component.wasm 
```

# Host

```shell
cargo run
```