# Hello world in Rust using libc in 904 bytes

Building:

```$ cargo b --release```

Striping using [ELF kickers](https://github.com/BR903/ELFkickers)’ `sstrip`:

```$ sstrip target/x86_64-unknown-linux-gnu/release/helloworld_libc```