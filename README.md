# Mars

A rust based Ray tracer following the Ray Tracing in a weekend book.

Runs with nightly rustc

# Build instructions

## Dependencies

Mars depends on `libXcursor`. Install it using your system's package manager.

Ubuntu:

```
$ sudo apt install libXcursor-dev
```

Fedora:

```
$ sudo dnf install libXcursor-devel
```

## Building with Rust

1. Install rustup/rust/cargo from https://rustup.rs/
2. Run `rustup install nightly` (or `rustup update nightly`)
3. Run `rustup override set nightly` while in the `mars/` directory
4. Run `cargo run --release`
5. Enjoy