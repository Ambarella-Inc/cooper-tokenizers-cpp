# Building the cooper-tokenizers-cpp Library

> **Important:** This document describes how to build the static library
> natively on an x86_64 Ubuntu host and cross-compile it for AArch64 Linux.
> Sections 1–3 require the AArch64 cross-compilation toolchain.

## 1. Install and verify the Rust toolchain

Install `rustup` if it is not already available:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "${HOME}/.cargo/env"
```

The build is pinned to Rust 1.87.0. This toolchain includes the x86_64 Ubuntu
host target. Add only the AArch64 cross-compilation target:

```bash
rustup toolchain install 1.87.0
rustup target add --toolchain 1.87.0 aarch64-unknown-linux-gnu
```

Verify the installation:

```bash
cargo +1.87.0 --version
rustup target list --toolchain 1.87.0 --installed
```

The output of the final command must include:

```text
aarch64-unknown-linux-gnu
```

Ensure the x86_64 compiler and the AArch64 cross compiler is available. For example:

```bash
command -v gcc
command -v <path-to-aarch64-toolchain-bin>/aarch64-linux-gnu-gcc
```

Do not continue until all commands succeed.

## 2. Clone the source repository from ambarella github

Clone the latest code from the `cooper` branch of the designated repository:

```bash
git clone \
  --branch cooper \
  --single-branch \
  https://github.com/Ambarella-Inc/cooper-tokenizers-cpp.git
cd cooper-tokenizers-cpp/rust
```

The repository must already contain the approved modifications and the
`Cargo.lock` file under the rust directory; no fork creation or patch application is needed.

## 3. Build x86_64 and AArch64

Run both builds from the `rust/` directory with the locked dependency set.

### 3.1 Build natively for the x86_64 Ubuntu host

Do not specify `--target`; Cargo uses the host target installed with Rust
1.87.0:

```bash
cargo +1.87.0 build \
  --release \
  --locked
```

### 3.2 Cross-compile for AArch64 Linux

Configure native-code dependency build scripts to use the AArch64 cross compiler:

```bash
export CC_aarch64_unknown_linux_gnu="<path-to-aarch64-toolchain-bin>/aarch64-linux-gnu-gcc"

cargo +1.87.0 build \
  --release \
  --locked \
  --target aarch64-unknown-linux-gnu
```

The expected outputs are:

```text
target/release/libtokenizers_c.a
target/aarch64-unknown-linux-gnu/release/libtokenizers_c.a
```

The updated `libtokenizers_c.a` is now ready for use.
