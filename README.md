# Network Ebpf

This project was built to learn and implement simple programs related to:

- Linux kernel eBPF
- Solana development with Anchor framework
- Run a Solana binary within `solana_rbpf` VM

## Aya project setup

### Setup a development environment[^dev]

```sh
rustup install stable
rustup toolchain install nightly --component rust-src
cargo install bpf-linker
cargo install cargo-generate
# cargo generate https://github.com/aya-rs/aya-template
# cargo generate --name myapp -d program_type=xdp https://github.com/aya-rs/aya-template
cargo generate --name xdp-drop-limit -d program_type=xdp https://github.com/aya-rs/aya-template
```

### Compile eBPF program

```sh
cargo xtask build-ebpf
```

To verify the program:

```sh
# llvm-objdump -S target/bpfel-unknown-none/debug/myapp
llvm-objdump -S target/bpfel-unknown-none/debug/xdp-drop-limit
```

### User-space Component

Build:

```sh
cargo build
```

Run:

```sh
# RUST_LOG=info cargo xtask run -- --iface [interface]
RUST_LOG=info cargo xtask run -- --iface wlp1s0
```

Check if eBPF program is running:

```sh
sudo bpftool prog list
```

## References:

[^1]: https://aya-rs.dev/book/ "The Aya Book"
[^2]: https://github.com/aya-rs/awesome-aya "Awesome Aya"
[^dev]: https://aya-rs.dev/book/start/development "The Aya Book: Development Environment"

