# Overview

Password generator in rust. Based on [Diceware](https://diceware.dmuth.org/) and the [Electronic Frontier Foundation](https://eff.org)'s large wordlist. 

## Build for Windows
```sh
sudo apt install mingw-w64               # download windows arch linker

rustup target add x86_64-pc-windows-gnu  # download rust components for arch

cargo build --target x86_64-pc-windows-gnu --release # build project w/ target arch
# builds to /target/x86_64-pc-windows-gnu/release
```