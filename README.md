#  Lox Programming Language — Rust

> A fully-featured bytecode VM for the **Lox** programming language, built from scratch in Rust.
> Inspired by Robert Nystrom's [*Crafting Interpreters*](https://craftinginterpreters.com/).

[![Rust](https://img.shields.io/badge/Rust-1.78%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

---

##  Overview

This project is a handcrafted implementation of the **Lox** scripting language interpreter in Rust. Lox is a dynamically-typed, object-oriented language designed by Robert Nystrom as a teaching vehicle for interpreter design. This implementation covers the full pipeline from raw source text to evaluated output — scanner, parser, AST, Optimized Bytecode,  — all written in idiomatic Rust.

---

##  Architecture

```
Source Code (.lox)
       │
       ▼
 ┌──────────────┐
 │    Scanner   │  → Tokenizes raw source into a stream of Tokens
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │    Parser    │  → Builds an Abstract Syntax Tree (AST) from tokens
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │  Compiler    │  → Translates AST source to LLVM IR Code 
 └──────┬───────┘
        │
        ▼
 ┌──────────────┐
 │  Bytecode VM │  → Stack-based execution engine; processes Bytecode
 └──────────────┘
        │
        ▼
Targeted Binary 
```

---

##  Features Implemented

### Lexer / Scanner
- Full tokenization of Lox source files
- Handles all literal types: numbers, strings, booleans, `nil`
- Identifier and reserved keyword recognition
- Robust UTF-8 character processing
- Meaningful lexer-level error messages with line context

### Parser
- Recursive descent parser for Lox grammar
- Full expression parsing (arithmetic, comparison, logical, grouping)
- Structured error handling and panic-mode recovery
- Generates a complete, well-formed Abstract Syntax Tree (AST)


## Roadmap

This interpreter is actively being developed. Upcoming milestones include:

| Feature | Status |
|---|---|
| Scanner | ✅ Complete |
| Expression Parser | ✅ Complete |
| AST Evaluation | ✅ Complete |
| Statement Parser (`print`, `var`, blocks) | 🔄 In Progress |
| Variables & Environments | 🔜 Planned |
| Control Flow (`if`, `while`, `for`) | 🔜 Planned |
| Functions & Closures | 🔜 Planned |
| Classes & Inheritance | 🔜 Planned |
| Bytecode VM | 🔜 Planned |

---

##  Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.78+)
- Cargo (bundled with Rust)

### Build

```bash
git clone https://github.com/Cletine/lox-interpreter.git
cd lox-interpreter
cargo build --release
```

### Run

```bash
# Run a .lox source file
cargo run --bin lox_interpreter -- /path/to/your/src.lox

```

### Test

```bash
cargo test
```

---

##  Example Lox Code

```lox
// Variables and arithmetic
var a = 10;
var b = 20;
print a + b; // 30

// String concatenation
var greeting = "Hello, " + "Lox!";
print greeting;

// Conditional logic
if (a < b) {
  print "a is smaller";
} else {
  print "b is smaller";
}
```

> **Note:** Not all of the above features may be executable yet depending on the current stage of the interpreter. See the roadmap above.


##  Design Decisions

**Why Rust?**
Rust's ownership model makes it exceptionally well-suited for compiler toolchain development. Memory safety is guaranteed at compile time, and the type system enforces correctness across complex recursive data structures like ASTs — without a garbage collector. Rust also allows for functional programming features such as Algebraic Data Types with Structs and Enums as well as powerful pattern matching which makes the construction of complex intermediary types simpler to implement and allows for simpler data decomposition with specificity to shape. This allows for the Compiler Frontend to have a rudimentary (and hopefully intuitive) implementation.


---

##  Reference

-  [*Crafting Interpreters* by Robert Nystrom](https://craftinginterpreters.com/) — the definitive guide this project is based on
-  [The Rust Programming Language](https://doc.rust-lang.org/book/)
-  [Rust Reference](https://doc.rust-lang.org/reference/)
-  [My First Language Frontend with LLVM Tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html)

---

##  License

This project is licensed under the [MIT License](LICENSE).

---

<p align="center">
  <i>If you found this project interesting, consider giving it a ⭐ — it helps others find it!</i>
</p>
