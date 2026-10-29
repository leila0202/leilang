# LeiLang Compiler

A native compiler for the **LeiLang** programming language (`.lei`), built from scratch in Rust. This compiler bypasses intermediate representations like LLVM or C to emit native Windows x86-64 assembly, linking directly against the Universal C Runtime (UCRT64).

## Project Roadmap

- [x] **File I/O**: Read and validate `.lei` source files into memory.
- [ ] **Lexer**: Tokenize the raw character stream, tracking metadata and source spans.
- [ ] **Parser**: Build an Abstract Syntax Tree (AST) from the token stream.
- [ ] **Code Generation**: Emit native Windows x86-64 Assembly.
- [ ] **Assembler & Linker Integration**: Generate final executables using native Windows tools.

## Architecture

To maximize the educational value of building a compiler, this project is designed and implemented completely by hand, avoiding external compiler-generation tools or lexer frameworks. 

### Core Compilation Pipeline

TBD

## Development Environment

- **Language**: Rust
- **IDE**: Visual Studio Code
- **Extensions**: Rust profile with `rust-analyzer`

## Getting Started

### Prerequisites

- Rust toolchain (`cargo`, `rustc`)
- Windows environment with UCRT64 support

### Running the Compiler

To compile a `.lei` source file, place your code in `test.lei` and run the compiler via Cargo:

```bash
cargo run
```
