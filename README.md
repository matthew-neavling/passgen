# Overview

Password generator in rust. Based on [Diceware](https://diceware.dmuth.org/) and the [Electronic Frontier Foundation](https://eff.org)'s large wordlist. 

## Usage
```sh
passgen
# r + enter to reroll
# TamenessGibletMotive
```

```sh
passgen 5 
# r + enter to reroll
# SteamLucidShrineKaratePaddle
```

## Build for Windows
General instructions for cross-compiling for Windows on Unix.
```sh
sudo apt install mingw-w64 
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
```