# Rust

Rust Version - 1.47.0
Cargo Version - 1.47.0

This repository contains:

- All the examples of the book **"The Rust Programming Language"** by Steve Klabnik and Carol Nichols, with contributions from the Rust Community;
- Examples of some other Rust tests.

### Rust Overview

Rust is a programming language that wants to avoid the "old pitfalls and providing a friendly, polished set of tools". It can be used within lower-level control systems with low-level code but is also strong enough to make CLI apps, web servers, and many other kinds of software. Rust allow us to "dip-down" without taking risk of crashes and/or security holes. It's also very efficient when we talk about speed and memory usage. The Rust compiler can detect and refuse to compile code with bugs including concurrency bugs. 

### Rust Developer Tools

- **Cargo** - dependency manager and build tool;
- **Rusfmt** - ensures a consistent coding style across developers;
- **Rust Language Server** - Integrated Development Environment (IDE) integration for code completion and inline error messages.

### Rust Installation

#### macOS

To install Rust on macOS you can do it by using the terminal and run the following command:

``` curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh ```

or install it using Homebrew:

``` brew install rust  ```

### How to use Cargo

- To build a project: ```cargo build``` (Add --release flag when the project finishes to compile it with optimizations)
- To build and run a project (one step operation): ```cargo run```
- To build a project to cech for errors without producing a binary: ```cargo check```
- To update crates ```cargo update```

### Tasks Done from "The Rust Programming Language":

- [x] Hello World;
- [x] Hello World Cargo;
- [x] Guessing Game;

by João Santos :ghost: 