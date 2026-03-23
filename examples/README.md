# Examples

This directory contains examples that demonstrate how to use `yapcol` to build real-world parsers.

## Overview

We provide two different implementations of a simple arithmetic expression parser and evaluator. Both examples implement
the same grammar and logic, but they differ in how they process the input.

### [evaluate_expression_string.rs](evaluate_expression_string.rs)

This example parses the input directly: the input string is converted into a stream of characters (`char`) and passed to
the parser. Its parsing logic is a bit complex because it must handle both lexical and syntactic structures.

### [evaluate_expression_token.rs](evaluate_expression_token.rs)

This example performs lexical analysis (a.k.a. lexing, tokenization) on the input string before sending it to the
parser. The parser's input is a stream of `Token` (a user-defined type) instead of a stream of characters. This approach
leverages separation of concerns: lexical and syntactic structures are analyzed at different stages.

## Running the Examples

Each example is its own binary crate that can be executed using `cargo run --example <name>`:

```bash
# Run string-based example 
cargo run --example evaluate_expression_string

# Run token-based example 
cargo run --example evaluate_expression_token
```

### Supported features

The examples are interactive command-line applications that query the user for arithmetic expressions that use (a
combination of) the following:

- Addition and subtraction (e.g., `5+32-1`).
- Multiplication and division (e.g., `96/4*3`).
- Exponentiation (e.g., `2^3`).
- Parenthesis to group expressions, possibly changing the precedence of operators (e.g., `(4+3)*2`).

### Usage

Once the application starts, it requests an expression as input:

```bash
Enter expression, or 'q' to quit
```

The user can then type an expression and submit it by pressing the Enter/⏎ key. The application will then attempt
to parse and evaluate the provided expression. If it succeeds, the value of the expression is displayed:

```bash
Enter expression, or 'q' to quit:
(4+3)*2
Success: 14
```

If it fails, an error will be displayed:

```bash
Enter expression, or 'q' to quit:
4*banana
Failed to parse expression: UnexpectedToken
```

The application keeps requesting new input expressions indefinitely, or until the user requests it to quit by providing
`q` as input.

## Common

The examples share code inside the `expression` module. It contains:

- The `Expression` and `Operator` enums.
- The `evaluate` function.

## Grammar

Both examples implement the same grammar, described here
in [Backus–Naur Form (BNF)](https://en.wikipedia.org/wiki/Backus–Naur_form):

```bnf
<expression>  ::= <term>
                | <expression> "+" <term>
                | <expression> "-" <term>
<term>        ::= <factor>
                | <term> "*" <factor>
                | <term> "/" <factor>
<factor>      ::= <exponential>
                | <exponential> "^" <factor>
<exponential> ::= <number>
                | "(" <expression> ")"
<number>      ::= <digit>
                | <number> <digit>
<digit>       ::= "0" | "1" | "2" | "3" | "4" 
                | "5" | "6" | "7" | "8" | "9"
```

Where:

- Operator precedence follows the given order: `+`/`-` < `*`/`/` < `^`.
- Parenthesis can be used to change the operator precedence.
- Addition, subtraction, multiplication and division are left-associative.
- Exponentiation is right-associative.

## Running Tests

You can run the unit tests for each example to see how individual components are verified:

```bash
# Test string-based example
cargo test --example evaluate_expression_string

# Test token-based example
cargo test --example evaluate_expression_token
```