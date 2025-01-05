# Notes

## Chaper #1

### Rust

- An ahead-of-time compiled language unlike dynamic languages like JS, Python or Ruby
- Statically type language (which means that it must know the
types of all variables at compile time)

### Conventions
- Use hello_world.rs rather than helloworld.rs.

### Commands
```console
// Commands to compile & run:
1. rustc main.rs
2. ./main

// Create project with cargo
cargo new hello_cargo

// Create Cargo.toml file
cargo init

// Build & run project
cargo build
./target/debug/hello_cargo

// --release flag to compile with optimizations

// Do it using only one command
cargo run

// Doesn't produce executable and only checks if code compiles
// Why?
// Faster way to check during coding
cargo check
```

### Cargo
- package manager like npm 
- manages dependencies
- packages of code are referred to as crates
- cargo.toml -> package.json of node

## Chapter 3

- By default, variables are immutable. Why? 
    - Safety and easy concurrency



### Left Chapters
- Chapter 2


### Missing?
- Initial pages why rust?
