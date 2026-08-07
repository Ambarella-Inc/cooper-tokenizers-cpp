# Modifications to tokenizers-cpp

## Upstream baseline

- Project: [mlc-ai/tokenizers-cpp](https://github.com/mlc-ai/tokenizers-cpp)
- License: Apache License, Version 2.0
- Base commit: `0621f84c1aed671d97cfd6eddd127809a20bc94f`
- Base commit date: February 6, 2025
- Modifier: `Ambarella International LP.`
- Modification year: 2026

## Purpose

The fork extends the C API with decoder-aware token-ID conversion. The new API
can apply the configured decoder pipeline and report the raw byte represented
by a ByteFallback token such as `<0xNN>`.

## Files changed from the upstream baseline

### `include/tokenizers_c.h` — modified

- Added `INVALID_RAW_WORD` with value `0xFFFFFFFF`.
- Declared `tokenizers_id_to_token_ext(...)`.
- Added a prominent modification notice while retaining the upstream
  copyright notice.

### `rust/src/lib.rs` — modified

- Imported `lazy_static`, `Decoder`, and `DecoderWrapper`.
- Added the `bytes_char()` and `CHAR_BYTES` ByteLevel conversion helpers.
- Implemented the exported `tokenizers_id_to_token_ext(...)` function.
- Added handling for Sequence, Strip, ByteFallback, ByteLevel, and other
  configured decoder wrappers.
- Added a prominent modification notice.

### `rust/Cargo.toml` — modified

- Updated `tokenizers` from `0.20.0` to `0.20.1`.
- Added `lazy_static = "1.5"`.
- Added a prominent modification notice.

### `rust/Cargo.lock` — added

- Added the Cargo-generated lock file used for reproducible dependency
  resolution.

### `BUILD.md` — added

- Added instructions for native x86_64 and cross-compiled AArch64 release
  builds using the pinned Rust toolchain and locked dependencies.

### `MODIFICATIONS.md` — added

- Added this record of the upstream baseline, the files changed by the fork,
  and the purpose of each change.
- Documented the Apache License 2.0 redistribution requirements and
  third-party dependency compliance considerations.

## Apache License 2.0 handling

The fork must retain the upstream `LICENSE` file and all upstream copyright,
patent, trademark, and attribution notices. The three modified source files
listed above carry notices stating that they were changed, as required by
Section 4(b) of the Apache License, Version 2.0.

Place this file at the fork repository root. Do not remove the original
`Copyright (c) 2023 by Contributors` notice.

## Dependency compliance

The static library contains third-party Rust dependencies resolved by
`rust/Cargo.lock`. Apache-2.0 compliance for tokenizers-cpp does not by itself
satisfy the licenses of all transitive dependencies. Generate and review a
complete third-party license inventory for the release build, and distribute
all license texts and notices required by those dependencies.
