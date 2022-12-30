# Resurgence
![Test](https://github.com/StandingPadAnimations/Resurgence/workflows/Rust/badge.svg?branch=main)
![Test](https://github.com/StandingPadAnimations/Resurgence/workflows/DevSkim/badge.svg?branch=main)
[![Crates.io version shield](https://img.shields.io/crates/v/resurgence.svg)](https://crates.io/crates/resurgence)
[![Docs](https://docs.rs/logos/badge.svg)](https://docs.rs/resurgence/)
[![Crates.io license shield](https://img.shields.io/crates/l/resurgence.svg)](https://crates.io/crates/resurgence)


Join the Discord server!

[![Discord](https://badgen.net/badge/icon/discord?icon=discord&label)](https://discord.gg/e2GuJ2k6na)

Resurgence aims to be an embeddable virtual machine with an easy-to-use API for projects like:
* Game engines
* Interpreters for programming languages
* Cross-platform plugin interfaces
* Applications that want custom scripting behavior
* Applications that want to execute custom user code securely

Right now, Resurgence is currently in early stages of development, and shouldn't be used in any serious projects yet.

Unlike other projects, Resurgence is not designed to be used with a specific programming language. Instead, it is designed for others to create high-level programming languages that execute within Resurgence. If you are interested in developing one of these languages, we recommend reading [Crafting Interpreters](https://craftinginterpreters.com/) to get an idea on how to make one.

Resurgence also does not provide a standard library interface. This allows applications that use Resurgence to define their own.

## Features
* Register-based VM (executes faster than Stack-based VMs)
* Lightweight and simple
* Easy to embed in other projects
* Built-in security sandbox
* Modular ecosystem

## Example Usage in an Application
```rust
use resurgence::{bytecode, CodeHolder, Interpreter, ResurgenceState};
use std::io::Error;

fn print_string(state: &mut ResurgenceState) -> Result<(), Error> {
    let s = state.get_string()?;
    println!("{}", s);
    Ok(())
}

fn main() {
    // Open and read a bytecode file
    let code: CodeHolder = bytecode::read_bytecode_file("hello-world.rvm").unwrap();
    let mut it = Interpreter::from(code);

    // Give the Interpreter access to our print_string function
    it.register_function(print_string, "printString".to_string());

    // Execute a bytecode function called "main"
    it.execute_function(&"main".to_string()).unwrap();
}
```


## Architecture
![Architecture](images/architecture.png)

## Security
Our goal for security is to make the only vulnerability be the programmer using the crate (in other words, only the code written by programmers using this library should be of any concern, and the programmer should focus on the security of the code they write without having to worry about library side security). That's why we try to minimize the API as much as possible (this also allows for extreme flexibility).

Now one can't get rid of security vulnerabilities entirely, so we encourage developers to figure out ways to break security, report them, and help come up with solutions. We believe the best way to minimize security issues is to 1. encourage people to find security flaws, 2. make it easy to report those flaws, and 3. allow community involvement in fixing those issues.

## Building Docs
To get basic documentation, run:

`cargo doc --open`

If you want documentation for internal implementation stuff (yes, we document the internals), run:

`cargo doc --open --document-private-items`
