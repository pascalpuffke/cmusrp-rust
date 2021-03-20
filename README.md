# cmusrp-rust
External Discord Rich Presence provider for cmus, written in Rust.

![Preview image](preview.png)

This is a more minimal version of my [cmus-rich-presence Java program](https://github.com/MineClashTV/cmus-rich-presence).

Since this is my very first Rust program and I'm still pretty terrible at the language, I would recommend using the Java version linked above if you don't mind the extra resource usage.
However, I will try to bring this to a similar level of functionality eventually!

## Building
You know how it goes.

1. ```cargo build``` compiles this program into an executable located in ./target/release/crp-rust
2. ```cargo install --path .``` installs this program to ~/.cargo/bin/crp-rust

## Usage
Execute the program, put it in an autostart script or whatever.

There are no commandline options yet.
