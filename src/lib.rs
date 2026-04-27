//! YAPCoL (Yet Another Parser Combinator Library) is a flexible and simple-to-use
//! parser combinator library for Rust.
//!
//! It allows you to build complex parsers by combining smaller, simpler ones.
//! The library is designed to be straightforward, while still providing powerful features like
//! arbitrary lookahead and nested parsers.
//!
//! # Core Concepts
//!
//! - [`Parser`]: The central trait of the crate. Any function that takes a mutable reference
//!   to an [`Input`] and returns a `Result<Output, Error>` is a parser.
//! - [`Input`]: A wrapper around an iterator that provides buffering, lookahead, and position
//!   tracking capabilities.
//! - Combinators: Functions that take one or more parsers and return a new, more complex
//!   parser. Examples: [`is()`], [`many0()`], [`option()`], [`chain_left()`].
//!
//! # Features
//!
//! - Arbitrary Lookahead: backtrack and try alternative parsers using [`attempt()`] and
//!   [`look_ahead()`].
//! - Generic Input: works with any iterator whose elements implement the [`InputToken`] trait.
//! - Position Tracking: every token carries a [`input::Position`] (line and column).
//!   Parse errors include the position of the offending token, making it easy to produce
//!   human-readable error messages.
//!
//! # Quick Start
//!
//! ```
//! use yapcol::{Input, is, many0};
//!
//! let mut input = Input::new_from_chars("aaab".chars(), None);
//!
//! // Combine `is` and `many0` to parse multiple 'a's
//! let is_a = is('a');
//! let parser = many0(&is_a);
//!
//! let result = parser(&mut input);
//! assert_eq!(result, Ok(vec!['a', 'a', 'a']));
//! ```
//!
//! # Error Handling
//!
//! Every parser returns a `Result<Output, Error>`. When parsing fails, the `Err` variant contains
//! one of two possible errors, defined in the [`Error`] enum:
//!
//! - [`Error::UnexpectedToken`]`(Option<String>, Position)`: the
//!   parser encountered a token that did not satisfy its requirements. The first field is an
//!   optional source name (e.g., a file name), and the second is the [`input::Position`] (line and
//!   column) where the unexpected token was found.
//! - [`Error::EndOfInput`]: the input stream was exhausted before the parser could match.
//!
//! The code below showcases both error variants in a simple character-based parsing example:
//!
//! ```
//! use yapcol::input::Position;
//! use yapcol::{Error, Input, Mismatch, any, is};
//!
//! let source_name = Some(String::from("file.txt"));
//! let mut input = Input::new_from_chars(vec!['a'], source_name.clone());
//!
//! // Fails with UnexpectedToken when the token does not match.
//! let output = is('b')(&mut input);
//! let mismatch = Mismatch::with_expectation('b', 'a');
//! assert_eq!(
//! 	output,
//! 	Err(Error::UnexpectedToken(
//! 		source_name,
//! 		Position::new(1, 1),
//! 		Some(mismatch)
//! 	))
//! );
//!
//! // Consume the only token, then try to read more.
//! is('a')(&mut input).unwrap();
//! assert_eq!(any()(&mut input), Err(Error::EndOfInput(None)));
//! ```
//!
//! The [`Error`] type implements [`std::fmt::Display`], so you can easily print human-readable
//! error messages.
//!
//! ```
//! use yapcol::Error;
//! use yapcol::input::Position;
//!
//! let error = Error::UnexpectedToken(Some("file.txt".to_string()), Position::new(3, 12), None);
//! assert_eq!(error.to_string(), "Unexpected token at file.txt:3:12.");
//!
//! let error = Error::EndOfInput(None);
//! assert_eq!(error.to_string(), "End of input reached.");
//! ```
//!
//! # Examples
//!
//! YAPCoL has two crates in the `examples` directory that demonstrate the library's capabilities.
//! Both of them implement the same application: a simple arithmetic expression parser and
//! evaluator. Each example uses a slightly different implementation to achieve the task:
//!   - `evaluate_expression_string` uses a parser that takes a stream of *characters* as input.
//!     This example parses the input string directly into the custom `Expression` type.
//!   - `evaluate_expression_token` uses a parser that takes a stream of user-defined *tokens* as
//!     input. This example first performs lexical analysis (lexing) to turn the input string into
//!     a vector of tokens, then parses the token stream into the custom `Expression` type.
//!
//! These two approaches reflect real-world usage of parsers, which might parse text directly or
//! perform lexical analysis beforehand. Check the `README` file in the `examples` directory for
//! more information.

mod combinators;
mod error;
pub mod input;
mod parser;

pub use combinators::*;
pub use error::{Error, Mismatch};
pub use input::{CharToken, Input, InputToken, StringInput};
pub use parser::{Parser, StringParser};
