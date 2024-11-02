# How to flash

In order to flash the thing correctly, first must run

`cargo run --bin blinky`

then run

`sudo /home/vscode/.pico-sdk/picotool/2.0.0/picotool/picotool load '/workspaces/myFirstBlinky/MyFirstRustBlinky/rust_blinky2/target/thumbv6m-none-eabi/debug/blinky.elf' -fx`

## Other notes

It is worth noting that `.cargo/config.toml` is definitely a different file from `cargo.toml`.

However, my expectations of things being in cargo.toml are not unwarrented:

https://github.com/rust-lang/cargo/issues/12738

https://internals.rust-lang.org/t/proposal-move-some-cargo-config-settings-to-cargo-toml/13336/16

So currently Rust is experiencing some Crappy Package Management. A far cry from my favorite npm.

Mostly looking at rustflags, which apparently are being used to configure crate-specific options, which I can't set in cargo.toml.