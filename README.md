# min-no-std

A minimal setup for a `#![no_std]` library with `wee_alloc` as global allocator

## Environment Setup

1. Visit <https://www.rust-lang.org/tools/install> to install the stable rust toolchain for your system.

2. Install nightly rust

```bash
rustup toolchain install nightly
```

3. Update all installed rust toolchains

```bash
rustup update
```

4. Set the nightly channel as the default

```bash
rustup override set nightly
```

5. Install the `wasm32` rust build target

```bash
rustup target add wasm32-unknown-unknown
```
