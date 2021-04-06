# MS-Röj

*A minesweeper game*

First, you need to setup [gtk](https://www.gtk.org/docs/installations/) for your environment.<br/>
The rust version I used was 1.42.0-nightly (**old**)

## build

build using rust/cargo:
```sh
cargo run
# or
cargo build --release
```

If you want to install the application (on linux), use:
```sh
make install
```

... or OSX:
```sh
make app
```

**note**

`src/lib/application.rs` could need a rewrite..
