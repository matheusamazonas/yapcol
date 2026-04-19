# YAPCoL

**Y**et **A**nother **P**arser **Co**mbinator **L**ibrary. [![CI](https://github.com/matheusamazonas/yapcol/actions/workflows/ci.yaml/badge.svg)](https://github.com/matheusamazonas/yapcol/actions)
[![Crates.io](https://img.shields.io/crates/v/yapcol)](https://crates.io/crates/yapcol)

`yapcol` is a flexible and simple-to-use parser combinator library for Rust. It allows you to build complex parsers by
combining smaller, simpler ones. The library is designed to be easy to understand and use, while still providing
powerful features like arbitrary lookahead and nested parsers.

## Features

- **Arbitrary Lookahead**: easily backtrack and try alternative parsers using `attempt` and `look_ahead`.
- **Generic Input**: works with any iterator whose items implement the `Token` trait.
- **Zero Dependencies**: `yapcol` has no dependencies besides Rust's standard library.

## Installation
Add YAPCoL to your `Cargo.toml`:
```toml
[dependencies]
yapcol = "0.2.0"
```

Or use `cargo add`:
```shell
cargo add yapcol
```

## Supported Combinators

`yapcol` provides a wide range of built-in combinators:

- **Basic**: `is`, `satisfy`, `any`, `end_of_input`.
- **Choice and Optional**: `choice`, `option`, `maybe`.
- **Repetition**: `many0`, `many1`, `count`, `many_until`, `separated_by0`, `separated_by1`.
- **Lookahead and Backtracking**: `attempt`, `look_ahead`, `not_followed_by`.
- **Grouping**: `between`.
- **Associativity**: `chain_left`, `chain_right`.

## Usage

### Using Combinators

The most convenient approach to YAPCoL is to use the built-in combinators to create your parsers:

```rust
use yapcol::input::core::Input;
use yapcol::{is, many0};

let mut input = Input::new_from_chars("aaab".chars(), None);

// Combine 'is' and 'many0' to parse multiple 'a's
let is_a = is('a');
let parser = many0(&is_a);

let result = parser(&mut input);
assert_eq!(result, Ok(vec!['a', 'a', 'a']));
```

### Using Custom Parsers

You might also define your own custom parsers as functions. Any function of the following `Fn` trait
automatically implements the `Parser` trait:
```rust
Fn(&mut Input<IT>) -> Result<O, Error>
```
For example:
```rust
use yapcol::input::string::StringInput;
use yapcol::error::Error;
use yapcol::is;

fn my_custom_parser(input: &mut StringInput) -> Result<String, Error> {
	let a = is('a')(input)?;
	let b = is('b')(input)?;
	Ok(format!("{}{}", a, b))
}

let mut input = StringInput::new_from_chars("ab".chars(), None);
assert_eq!(my_custom_parser(&mut input), Ok("ab".to_string()));
```

## Error Handling

Every parser returns a `Result<O, Error>`. When parsing fails, the `Err` variant contains one of two possible errors,
defined in the `yapcol::error::Error` enum:

- `UnexpectedToken(Option<String>, Position)`: the parser encountered a token that did not satisfy its requirements.
  The first field is an optional source name (e.g., a file name), and the second is the `Position` (line and column)
  where the unexpected token was found.
- `EndOfInput`: the input stream was exhausted before the parser could match.

The code below showcases both error variants in a simple character-based parsing example:
```rust
use yapcol::{is, any};
use yapcol::error::Error;
use yapcol::input::core::Input;
use yapcol::input::position::Position;

let source_name = Some(String::from("file.txt"));
let mut input = Input::new_from_chars(vec!['a'], source_name.clone());

// Fails with UnexpectedToken when the token does not match.
assert_eq!(is('b')(&mut input), Err(Error::UnexpectedToken(source_name, Position::new(1, 1))));

// Consume the only token, then try to read more.
is('a')(&mut input).unwrap();
assert_eq!(any()(&mut input), Err(Error::EndOfInput));
```

The `Error` type implements `Display`, so you can easily print human-readable error messages:

```rust
use yapcol::error::Error;
use yapcol::input::position::Position;

let error = Error::UnexpectedToken(Some("file.txt".to_string()), Position::new(3, 12));
assert_eq!(error.to_string(), "Unexpected token at file.txt:3:12.");

let error = Error::EndOfInput;
assert_eq!(error.to_string(), "End of input reached.");
```

## Examples

Real-world examples are available in the `examples/` directory, including an arithmetic expression evaluator. There are
two different implementations:

- String-based: parses text directly from a stream of characters.
- Token-based: uses a lexer to tokenize the input before parsing.

For more details on how to run and understand these examples, check the [Examples README](examples/README.md).

## Contributing

If you would like to report a bug, please create an [issue](https://github.com/matheusamazonas/yapcol/issues). If you
would like to contribute with bug fixing or small improvements, please open a Pull Request. If you would like to
contribute with a new feature (regardless if it's in the roadmap or
not), [contact the developer](https://matheusamazonas.net/contact.html).

## License

YAPCoL is distributed under the terms of the MIT license. For more information, check the [LICENSE](LICENSE.md) file in
this repository.