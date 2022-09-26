# Hello world in Rust with libc in 904 bytes ELF64

Building:

```$ cargo b --release```

Striping using [ELF kickers](https://github.com/BR903/ELFkickers)’ `sstrip`:

```$ sstrip target/x86_64-unknown-linux-gnu/release/helloworld_libc```