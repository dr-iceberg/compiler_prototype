# Compiler prototype

A compiler for a simple rust-looking language. Its my first attempt at writing a compiler hence the "prototype" in the name.

# Building

Dependencies:
- Rust 2024 edition, version 1.98.0 or higher
- llvm 18.1.0 development packages
- libffi
- libxml2
- on `x86_64-unknown-linux-gnu` the [mold](https://github.com/rui314/mold) linker


`LLVM_SYS_181_PREFIX` environment variable set to the root directory of the llvm development packages

The easiest way to fetch and set all of the above is to use the provided `flake.nix` and `flake.lock` files by running:
```
nix develop
```

When everything is set and downloaded run:
```
cargo b
```
to build the project.

# Usage

To compile the source file into a llvm module:
```
cargo r -- src.txt
```
or directly:
```
./target/debug/compiler_prototype src.txt
```

to compile the source file into a llvm module that can be passed to clang with:
```
clang main.ll
```
which will output the executable.

# Missing features

The project is still not finished and there are some missing features I would like to add:

- Proper symbol name resolution in semantic analysis
- Internally invoking clang to produce the executable
- Better CLI interface
- Better error messages
- Cross compiling




