# rucompiler-x86

`rucompiler-x86` is a small educational compiler written in Rust. It translates a compact, C-like language into GNU-style x86-64 assembly and exposes every compiled program as a function named `foo` that can be linked with a C test harness.

The repository contains two compiler stages:

| Stage | Source | Input | Output | Purpose |
| --- | --- | --- | --- | --- |
| `first` | `first/src/main.rs` | One arithmetic expression | `<input>.s` | LL(1) parsing, AST/DAG construction, simple IR, and x86-64 generation |
| `second` | `second/src/main.rs` | A complete `.rucomp` program | `<input-stem>_2.s` | Variables, assignments, conditionals, loops, and direct x86-64 generation |

## Features

- Hand-written lexer and parser in Rust
- Operator precedence for addition, subtraction, and multiplication
- Integer variables and up to six function arguments
- `if`/`else` branches and `while` loops, including nested control flow
- Integer comparisons: `<`, `<=`, `>`, `>=`, and `==`
- System V AMD64 calling convention and AT&T assembly syntax
- Example programs, generated assembly, and C harnesses in the repository root

## Requirements

- A recent stable [Rust toolchain](https://www.rust-lang.org/tools/install) with Cargo
- An x86-64 Linux environment
- GCC or Clang to assemble, link, and run the generated code

The checked-in `rucompiler-x86-first` and `rucompiler-x86-second` executables are prebuilt Linux x86-64 binaries. Building from source is recommended.

## Quick start

Build both stages from the repository root:

```bash
cargo build --manifest-path first/Cargo.toml
cargo build --manifest-path second/Cargo.toml
```

Compile one of the included `.rucomp` examples with the full compiler:

```bash
cargo run --manifest-path second/Cargo.toml -- test1.rucomp
```

This creates `test1_2.s`. Link it with the matching C harness and run it:

```bash
cc test1.c test1_2.s -o test1.out
./test1.out
```

Every generated assembly file defines `foo`. The C declaration must therefore use the same number and order of arguments as the source program's `args` declaration.

## Source language

A program has four parts in order:

1. An argument declaration
2. A local integer declaration
3. Zero or more statements
4. A return statement

For example:

```text
args a, b;
int x, result;

x = a + b;
if x >= 10 then
{
    result = x * 2;
}
else
{
    result = x - 1;
}

return result;
```

The supported constructs are:

```text
program      := arg-decl var-decl statement* return-statement
arg-decl     := "args" identifier ("," identifier)* ";"
var-decl     := "int" identifier ("," identifier)* ";"
statement    := assignment | if-statement | while-statement
assignment   := identifier "=" expression ";"
if-statement := "if" boolean "then" "{" statement* "}"
                "else" "{" statement* "}"
while-statement := "while" boolean "then" "{" statement* "}"
boolean      := "true" | "false" | expression rel-op expression
expression   := terms joined by "+" or "-"
term         := factors joined by "*"
factor       := identifier | integer | "(" expression ")"
rel-op       := "<" | "<=" | ">" | ">=" | "=="
```

Identifiers currently contain ASCII letters only. Arithmetic uses 64-bit integers, and relational comparisons are emitted as unsigned comparisons.

## Expression compiler

The first stage accepts a file containing an expression made from identifiers, non-negative integer literals, `+`, `*`, and parentheses:

```text
a * (b + 5)
```

Run it with:

```bash
cargo run --manifest-path first/Cargo.toml -- expression.exp
```

It writes `expression.s`. Although the reader accepts multiple non-empty lines, each line targets the same output path, so use one expression per input file.

Internally this stage demonstrates the following pipeline:

```text
source -> tokens -> LL(1) parse tree -> AST -> optimized DAG -> IR -> x86-64
```

The DAG merges repeated leaves and equivalent associative/commutative `+` and `*` subexpressions before assembly is generated.

## Project layout

```text
.
├── first/                 # Expression compiler crate
│   └── src/main.rs
├── second/                # Full language compiler crate
│   └── src/main.rs
├── test1.rucomp ...       # Example source programs
├── test1.c ...            # C harnesses that call generated `foo`
├── test1.s ...            # First-stage assembly samples
└── test1_2.s ...          # Full-compiler assembly samples
```

## Current limitations

- The backend targets x86-64 Linux and emits GNU/AT&T syntax.
- Functions are always named `foo` and return a 64-bit integer.
- More than six arguments are not supported.
- There is no type checker or semantic validation pass; malformed declarations or unknown variables may fail during parsing or code generation.

## Exploring the examples

The numbered examples exercise progressively richer combinations of arithmetic, comparisons, branching, and nested loops. A typical workflow is:

```bash
cargo run --manifest-path second/Cargo.toml -- test10.rucomp
cc test10.c test10_2.s -o test10.out
./test10.out
```

Inspect the corresponding `.rucomp`, `.c`, and generated `_2.s` files side by side to follow the translation from source language to machine-level control flow.
