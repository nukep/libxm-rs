# libxm-rs

[![Crates.io version](https://img.shields.io/crates/v/libxm.svg)](https://crates.io/crates/libxm)
[![Docs](https://docs.rs/libxm/badge.svg)](https://docs.rs/libxm/)

A binding of [libxm](https://github.com/Artefact2/libxm/) for Rust.

A small XM (FastTracker II Extended Module) player library. Designed
for easy integration in demos and such, and provides timing functions
for easy sync against specific instruments, samples or channels.

As with libxm, this library is released under the WTFPL license.

## [Documentation](https://docs.rs/libxm/)

Documentation is hosted here: https://docs.rs/libxm/

## Build requirements

If `libxm` is built locally (this is the default!), you must have a C compiler
on your system that supports the C11 standard.
If you don't wish to build locally, a shared library that you have pre-built
can be provided by following the steps below.

## Linking to a shared version of `libxm`
By default, `libxm-rs` statically links and compiles `libxm`.
This is to allow users to get started with the library more quickly.

If you wish to provide your own shared or custom version of `libxm`, you can
override the build step for `xm` in a `.cargo/config` file
(see https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts).

```toml
[target.x86_64-unknown-linux-gnu.xm]
rustc-flags = "-l xm"
```
