# cmusrp-rust
External Discord Rich Presence provider for cmus, written in Rust.

![Preview image](preview.png)

This is a more minimal version of my [cmus-rich-presence Java program](https://github.com/MineClashTV/cmus-rich-presence).

Since this is my very first Rust program and I'm still pretty terrible at the language, I would recommend using the Java version linked above if you don't mind the extra resource usage.
However, I will try to bring this to a similar level of functionality eventually!

## Usage
```crp-rust [flags] [options]```

Flags:

```-d```, ```--debug```     Disables rich presence and displays current status on stdout

```-h```, ```--help```      Prints help information

```-V```, ```--version```   Prints version information


Options:

```-i```, ```--interval <interval>``` Polling interval in which the program grabs current status, in milliseconds.
                                      [default: 1000]

```-b```, ```--bottom <format>```     Sets custom formatting for the bottom string
                                      [default: "{artist} - {album} ({date})"]
                                      
```-t```, ```--top <format>```        Sets custom formatting for the top string
                                      [default: "{title}"]

## Building
You know how it goes.

1. ```cargo build``` compiles this program into an executable located in ./target/release/crp-rust
2. ```cargo install --path .``` installs this program to ~/.cargo/bin/crp-rust
