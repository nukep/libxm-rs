# libxm-rs

A binding of [libxm](https://github.com/Artefact2/libxm/) for Rust.

A small XM (FastTracker II Extended Module) player library. Designed
for easy integration in demos and such, and provides timing functions
for easy sync against specific instruments, samples or channels.

As with libxm, this library is released under the WTFPL license.

**Documentation**: https://nukep.github.io/libxm-rs/libxm

## Linking to a shared version of `libxm`
By default, `libxm-rs` statically links and compiles `libxm`.
This is to allow users to get started with the library more quickly.

If you wish to provide your own shared or custom version of `libxm`, you can
override the build step for `xm` in a `.cargo/config` file
(see http://doc.crates.io/build-script.html#overriding-build-scripts).

```toml
[target.x86_64-unknown-linux-gnu.xm]
rustc-flags = "-l xm"
```
