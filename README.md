# Basic commands

## Install Rust installer

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Update Rust

```
rustup update
```

## Manage packages and project

* build your project with `cargo build`
* build your project without producing a binary to check for errors with `cargo check`
* run your project with `cargo run`
* test your project with `cargo test`
* build documentation for your project with `cargo doc`
* publish a library to crates.io with `cargo publish`
* build for release with `cargo build --release`