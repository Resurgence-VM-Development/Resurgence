# Resurgence
![Test](https://github.com/StandingPadAnimations/Resurgence/workflows/Rust/badge.svg?branch=main)
![Test](https://github.com/StandingPadAnimations/Resurgence/workflows/DevSkim/badge.svg?branch=main)
[![Crates.io version shield](https://img.shields.io/crates/v/resurgence.svg)](https://crates.io/crates/resurgence)
[![Docs](https://docs.rs/logos/badge.svg)](https://docs.rs/resurgence/)
[![Crates.io license shield](https://img.shields.io/crates/l/resurgence.svg)](https://crates.io/crates/resurgence)

[![Discord](https://badgen.net/badge/icon/discord?icon=discord&label)](https://discord.gg/e2GuJ2k6na)

Resurgence aims to be an embedable VM backend with an easy to use API for projects like:
* Game engines
* Full blown interpreters
* Applications that want custom scripting

Right now, Resurgence is in the alpha phase, so don't expect stability for now

Note: Resurgence is just a backend. This allows us to focus on making it good without having to worry about maintaining a front end. For making a full blown VM, we recommend [Crafting Interpters](https://craftinginterpreters.com/) to get an idea on how to make one. Eventually there will be a reference implementation for anyone that needs it

# Building Docs
To get basic documentation, run:

`cargo doc --open` 

If you want documentation for internal implementation stuff (yes, we document the internals), run:

`cargo doc --open --document-private-items`


