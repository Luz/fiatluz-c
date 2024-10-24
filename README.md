# fiatluz-c
A wrapper for fiatluz written in Rust.

## Features
- Creates a static library in Rust
- Calls cbindgen to create the C-header files
- Demo C-project to call the Rust functions
- Using valgrind to prove there are no memory leaks

## Dependencies for the c-project example
- valgrind

## Get this repo
```Bash
git clone --recursive https://github.com/Luz/fiatluz-c
```

## Usage
```Bash
cargo build --release
cd c-project
make
```

## Highly needed features
- determinism (same input, same output) and its platform independence
- easy usage of rust tool
- stability, correctness, performance
- optional delaunay triangulation

