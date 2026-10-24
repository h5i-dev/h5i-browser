# h5i-browser

A lightweight visual browser engine for coding agents, written in Rust. No
Chromium, no V8.

The engine is assembled rather than written: [Blitz](https://github.com/DioxusLabs/blitz)
owns the DOM and drives [Stylo](https://github.com/servo/stylo) for CSS,
`vello_cpu` rasterises on the CPU, and [Boa](https://github.com/boa-dev/boa)
runs page script. Every request is policy-checked and receipted before it
reaches the wire.

## Layout

- `crates/h5i-browser` — the engine (DOM, CSS, script, network, policy, receipts).
- `crates/h5i-error` — the shared error type.
- `crates/h5i-wire` — the data shapes for receipt rows and stored messages.
- `src/main.rs` — the `h5i-browser` binary, a thin wrapper over `cli::main`.

## Build

```sh
cargo build --release
```

The binary is `h5i-browser`. Run `h5i-browser --help` for the command line.
