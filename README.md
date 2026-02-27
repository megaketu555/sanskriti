## sanskriti

A small experimental language toolchain that lets you write programs using Sanskrit/Devanagari keywords and variable names, translates them to Lox-style code, and then **runs** them with a tree‑walking interpreter.

Internally it’s based on the Pratt parser and lexer from *Crafting Interpreters*, with:

- Extended identifier support for Devanagari characters
- A Sanskrit → Lox keyword translator
- A basic interpreter that evaluates the resulting AST

---

## Installation

```bash
# In this directory:
cargo build
```

Requires:

- Rust (stable)
- Windows (tested), but should be portable to other platforms.

---

## Usage

The CLI exposes three subcommands:

- **Tokenize**: show tokens for a source file
- **Parse**: show the parsed AST for a single expression
- **Run**: translate + parse + execute a whole program

### 1. Tokenize

```bash
cargo run -- tokenize example.sk
```

Prints each token (type, lexeme, value) line by line. Devanagari identifiers and keywords like `चर`, `यावद`, `यदि`, `कथय` are recognized correctly.

### 2. Parse

```bash
cargo run -- parse example.sk
```

Parses the input as a single expression and prints the AST (S‑expression form). This is mainly for debugging the parser.

### 3. Run

```bash
cargo run -- run example.sk
```

This pipeline:

1. Reads `example.sk`
2. Translates Sanskrit keywords to Lox keywords
3. Parses the whole file into a sequence of statements
4. Interprets the program

For the provided `example.sk`:

```lox
चर आरम्भ = 1;
चर सीमा = 5;

यावद (आरम्भ <= सीमा) {
  कथय आरम्भ;
  आरम्भ = आरम्भ + 1;
}

चर true_ध्वज = सत्य;

यदि (true_ध्वज) {
  कथय "ध्वज true है";
} अथ्वा {
  कथय "ध्वज false है";
}
```

You should see output similar to:

```text
1.0
2.0
3.0
4.0
5.0
ध्वज true है
```

---

## Language Features

Currently supported (after translation to Lox‑style code):

- **Values**: numbers, booleans, strings, `nil`
- **Variables**: `चर` ↔ `var`
- **Assignments**: `name = expr;`
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparisons**: `<`, `<=`, `>`, `>=`, `==`, `!=`
- **Logic**: `and`, `or`, `!`
- **Control flow**:
  - `यावद` ↔ `while`
  - `यदि` / `अथ्वा` ↔ `if` / `else`
- **Printing**: `कथय` ↔ `print`
- **Blocks**: `{ ... }`, with multiple statements and optional semicolons

Devanagari identifiers (variable names, etc.) are fully supported in the lexer and parser.

---

## Internal Layout

- `src/lex.rs` — Lexer (tokenizer), extended for Devanagari identifiers.
- `src/parse.rs` — Pratt parser that builds a `TokenTree` AST. Includes:
  - `parse_expression`
  - `parse_statement_within`
  - `parse_block`
  - `parse_program` (parses a whole file into a list of statements)
- `src/translator.rs` — Simple keyword‑level translator from Sanskrit to Lox.
- `src/interpreter.rs` — Tree‑walking interpreter over `TokenTree`:
  - `Interpreter::eval_program(&[TokenTree])`
- `src/main.rs` — CLI entrypoint with the `tokenize`, `parse`, and `run` subcommands.
- `src/banner.rs` — Startup banner.

---

## Limitations and Future Work

- No functions, classes, or closures yet (the parser has some scaffolding, the interpreter mostly ignores them).
- No error recovery: most syntax errors abort parsing with a diagnostic.
- The translator is a simple string replacement; it doesn’t yet handle more advanced syntax or context‑sensitive constructs.

Contributions and experiments (new keywords, control flow, or a richer runtime) are very welcome.

