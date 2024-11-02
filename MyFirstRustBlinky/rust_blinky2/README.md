# How to flash

In order to flash the thing correctly,

1. First must restart the dev container (Must do this EVERY TIME you want to flash),

2. must run

`cargo run`

3. Then rename

`rust_blinky2` to `rust_blinky2.elf`

4. then run

`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_blinky2/target/thumbv6m-none-eabi/debug/rust_blinky2.elf' -fx`

## Other notes

It is worth noting that `.cargo/config.toml` is definitely a different file from `cargo.toml`.

However, my expectations of things being in cargo.toml are not unwarrented:

https://github.com/rust-lang/cargo/issues/12738

https://internals.rust-lang.org/t/proposal-move-some-cargo-config-settings-to-cargo-toml/13336/16

So currently Rust is experiencing some Crappy Package Management. A far cry from my favorite npm.

Mostly looking at rustflags, which apparently are being used to configure crate-specific options, which I can't set in cargo.toml.

UPDATE 1:
I was able to remove the bin specification to use the default src/main from the cargo.toml, so now instead of `cargo run --bin blinky` I could run `cargo run`