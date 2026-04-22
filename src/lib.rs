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
//!   parser. Examples: [`is()`], [`many0`], [`option()`], [`chain_left`].
//!
//! # Features
//!
//! - Arbitrary Lookahead: backtrack and try alternative parsers using [`attempt()`] and
//!   [`look_ahead()`].
//! - Generic Input: works with any iterator whose items implement the [`InputToken`] trait.
//! - Position Tracking: every token carries a [`input::position::Position`] (line and column).
//!   Parse errors include the position of the offending token, making it easy to produce
//!   human-readable error messages.
//!
//! # Quick Start
//!
//! ```
//! use yapcol::input::core::{Input};
//! use yapcol::{is, many0};
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
//! Every parser returns a `Result<O, Error>`. When parsing fails, the `Err` variant contains
//! one of two possible errors, defined in the [`Error`] enum:
//!
//! - [`Error::UnexpectedToken`]`(Option<String>, Position)`: the
//!   parser encountered a token that did not satisfy its requirements. The first field is an
//!   optional source name (e.g., a file name), and the second is the [`input::position::Position`]
//!   (line and column) where the unexpected token was found.
//! - [`Error::EndOfInput`]: the input stream was exhausted before the parser could match.
//!
//! The code below showcases both error variants in a simple character-based parsing example:
//!
//! ```
//! use yapcol::{is, any};
//! use yapcol::error::Error;
//! use yapcol::input::core::Input;
//! use yapcol::input::position::Position;
//!
//! let source_name = Some(String::from("file.txt"));
//! let mut input = Input::new_from_chars(vec!['a'], source_name.clone());
//!
//! // Fails with UnexpectedToken when the token does not match.
//! assert_eq!(is('b')(&mut input), Err(Error::UnexpectedToken(source_name, Position::new(1, 1))));
//!
//! // Consume the only token, then try to read more.
//! is('a')(&mut input).unwrap();
//! assert_eq!(any()(&mut input), Err(Error::EndOfInput));
//! ```
//!
//! The [`Error`] type implements [`std::fmt::Display`], so you can easily print human-readable error
//! messages.
//!
//! ```
//! use yapcol::error::Error;
//! use yapcol::input::position::Position;
//!
//! let error = Error::UnexpectedToken(Some("file.txt".to_string()), Position::new(3, 12));
//! assert_eq!(error.to_string(), "Unexpected token at file.txt:3:12.");
//!
//! let error = Error::EndOfInput;
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

use crate::error::Error;
use crate::input::core::{Input, InputToken};
use crate::input::string::{CharToken, StringInput};

pub mod combinators;
pub mod error;
pub mod input;

pub use combinators::*;

/// The core trait of the `yapcol` crate, representing a parser.
///
/// A `Parser` is a function (or any type that implements `Fn`) that takes a mutable reference
/// to an [`Input`] and returns a `Result` containing either the successfully parsed output
/// of type `O` or an [`Error`].
///
/// This trait is automatically implemented for any function with the signature
/// `Fn(&mut Input<I>) -> Result<O, Error>`.
///
/// # Type Parameters
///
/// - `IT`: The parser's input token, which implements the [`InputToken`] trait.
/// - `O`: The type of the value produced by the parser on success.
///
/// # Examples
///
/// You can define a custom parser as a function:
///
/// ```
/// use std::str::Chars;
/// use yapcol::input::core::{Input};
/// use yapcol::input::string::StringInput;
/// use yapcol::error::Error;
/// use yapcol::is;
///
/// fn my_uppercase_parser(input: &mut StringInput) -> Result<char, Error> {
///    // You can use existing parsers inside your custom parser
///    is('A')(input)
/// }
///
/// let mut input = Input::new_from_chars("Abc".chars(), None);
/// assert_eq!(my_uppercase_parser(&mut input), Ok('A'));
/// ```
///
/// Most of the time, you will use the built-in combinators which return `impl Parser`:
///
/// ```
/// use yapcol::input::core::Input;
/// use yapcol::is;
///
/// let mut input = Input::new_from_chars("Abc".chars(), None);
/// let mut parser = is('A');
/// assert_eq!(parser(&mut input), Ok('A'));
/// ```
pub trait Parser<IT, O>: Fn(&mut Input<IT>) -> Result<O, Error>
where
	IT: InputToken,
{
	/// Transforms the output of the current parser using the provided function.
	///
	/// # Parameters
	/// - `self`: The current parser.
	/// - `f`: A closure or function that maps the previous output of the parser to a new output
	///   type.
	///
	/// # Returns
	/// A new parser that applies the mapping function `f` to the output of the current parser.
	///
	/// # Examples
	/// ```rust
	/// use yapcol::{Parser, satisfy, any};
	/// use yapcol::input::core::{Input};
	///
	///
	/// let is_digit = |c: &char| if c.is_ascii_digit() { Some(*c) } else { None } ;
	/// let parser = satisfy(is_digit).map(|c| c.to_digit(10));
	///
	/// let mut input = Input::new_from_chars("1".chars(), None);
	/// let result = parser(&mut input);
	/// assert_eq!(result, Ok(Some(1)));
	/// ```
	///
	/// # Errors
	/// If the current parser fails, the error is returned as is without invoking the mapping
	/// function `f`.
	fn map<F, MO>(self, f: F) -> impl Parser<IT, MO>
	where
		F: Fn(O) -> MO,
		Self: Sized,
	{
		move |input| match self(input) {
			Ok(value) => Ok(f(value)),
			Err(e) => Err(e),
		}
	}

	/// Applies a function to the output of the current parser to produce a new parser, then runs
	/// that new parser on the same input.
	///
	/// This is useful for chaining parsers where the next parser depends on the result of the
	/// previous one.
	///
	/// # Parameters
	/// - `self`: The current parser.
	/// - `f`: A closure or function that takes the output of the current parser and returns a new
	///   parser.
	///
	/// # Returns
	/// A new parser that first runs the current parser, then passes its output to `f` to obtain
	/// a second parser, and finally runs that second parser on the same input.
	///
	/// # Examples
	/// ```rust
	/// use yapcol::{Parser, is};
	/// use yapcol::input::core::Input;
	///
	/// // Parse 'a' twice.
	/// let twice_parser = is('a').and_then(is);
	/// let mut input = Input::new_from_chars("aa".chars(), None);
	/// assert_eq!(twice_parser(&mut input), Ok('a'));
	///
	/// let mut input = Input::new_from_chars("ac".chars(), None);
	/// assert!(twice_parser(&mut input).is_err());
	/// ```
	///
	/// # Errors
	/// If the current parser fails, the error is returned as is without invoking `f`. If the
	/// parser returned by `f` fails, its error is returned.
	fn and_then<F, P, NO>(self, f: F) -> impl Parser<IT, NO>
	where
		F: Fn(O) -> P,
		P: Parser<IT, NO>,
		Self: Sized,
	{
		move |input| match self(input) {
			Ok(value) => f(value)(input),
			Err(e) => Err(e),
		}
	}

	/// Runs the current parser, discards its output, then runs `other` on the same input and
	/// returns its result.
	///
	/// This is useful when you want to assert that a certain token or pattern is present without
	/// caring about its value, and then continue parsing with a second parser.
	///
	/// # Parameters
	/// - `self`: The current parser whose output is discarded on success.
	/// - `other`: The parser to run after `self` succeeds.
	///
	/// # Returns
	/// A new parser that first runs `self`, discards its output, then runs `other` on the same
	/// input and returns its result.
	///
	/// # Examples
	/// ```rust
	/// use yapcol::{Parser, is, any};
	/// use yapcol::input::core::Input;
	///
	/// // Skip 'a', then parse any character.
	/// let parser = is('a').and(any());
	/// let mut input = Input::new_from_chars("ab".chars(), None);
	/// assert_eq!(parser(&mut input), Ok('b'));
	///
	/// // Fails if the first parser fails.
	/// let parser = is('a').and(any());
	/// let mut input = Input::new_from_chars("xb".chars(), None);
	/// assert!(parser(&mut input).is_err());
	/// ```
	///
	/// # Errors
	/// If `self` fails, its error is returned and `other` is never run.
	fn and<P, NO>(self, other: P) -> impl Parser<IT, NO>
	where
		P: Parser<IT, NO>,
		Self: Sized,
	{
		move |input| match self(input) {
			Ok(_) => other(input),
			Err(e) => Err(e),
		}
	}

	/// A shortcut for the [`attempt::attempt`] combinator.
	fn attempt(self) -> impl Parser<IT, O>
	where
		Self: Sized,
	{
		move |input| attempt(&self)(input)
	}

	/// A shortcut for the [`maybe::maybe`] combinator.
	fn maybe(self) -> impl Parser<IT, Option<O>>
	where
		Self: Sized,
	{
		move |input| maybe(&self)(input)
	}

	/// A shortcut for the [`many0`] combinator.
	fn many0(self) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| many0(&self)(input)
	}

	/// A shortcut for the [`many1`] combinator.
	fn many1(self) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| many1(&self)(input)
	}
}

impl<IT, O, X> Parser<IT, O> for X
where
	X: Fn(&mut Input<IT>) -> Result<O, Error>,
	IT: InputToken,
{
}

/// A convenience alias for [`Parser`] specialised to character-stream input.
///
/// `StringParser<O>` is equivalent to `Parser<CharToken, O>` and is automatically implemented
/// for any function `Fn(&mut StringInput) -> Result<O, Error>`. It exists purely to reduce
/// type-annotation noise when working with string-based parsers.
pub trait StringParser<O>: Parser<CharToken, O> {}

impl<O, X> StringParser<O> for X where X: Fn(&mut StringInput) -> Result<O, Error> {}

#[cfg(test)]
mod tests {
	mod map {
		use crate::input::position::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('2').map(|c: char| c.to_digit(10));
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(parser(&mut input), Err(Error::EndOfInput));
		}

		#[test]
		fn success_simple() {
			let parser = is('2').map(|c: char| c.to_digit(10));
			let mut input = Input::new_from_chars("2".chars(), None);
			assert_eq!(parser(&mut input), Ok(Some(2)));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn success_chained() {
			let parser = is('2')
				.map(|c: char| c.to_digit(10))
				.map(|o| o.unwrap())
				.map(|x| x * 3);
			let mut input = Input::new_from_chars("2".chars(), None);
			assert_eq!(parser(&mut input), Ok(6));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn fail_simple() {
			let parser = is('2').map(|c: char| c.to_digit(10));
			let mut input = Input::new_from_chars("3".chars(), None);
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 1)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn fail_chained() {
			let parser = is('5')
				.map(|c: char| c.to_digit(10))
				.map(|o| o.unwrap())
				.map(|x| x * 7);
			let mut input = Input::new_from_chars("3".chars(), None);
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 1)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}

	mod and_then {
		use crate::input::position::Position;
		use crate::*;

		#[test]
		fn empty() {
			let double_parser = is('2').and_then(is);
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(double_parser(&mut input), Err(Error::EndOfInput));
		}

		#[test]
		fn success_simple() {
			let double_parser = is('2').and_then(is);
			let mut input = Input::new_from_chars("22".chars(), None);
			assert_eq!(double_parser(&mut input), Ok('2'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn success_chained() {
			let triple_parser = is('2').and_then(is).and_then(is);
			let mut input = Input::new_from_chars("222".chars(), None);
			assert_eq!(triple_parser(&mut input), Ok('2'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn fail_simple() {
			let double_parser = is('2').and_then(is);
			let mut input = Input::new_from_chars("23".chars(), None);
			assert_eq!(
				double_parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 2)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn fail_chained() {
			let triple_parser = is('2').and_then(is).and_then(is);
			let mut input = Input::new_from_chars("223".chars(), None);
			assert_eq!(
				triple_parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 3)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}

	mod and {
		use crate::input::position::Position;
		use crate::*;

		#[test]
		fn empty() {
			let double_parser = is('2').and(is('3'));
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(double_parser(&mut input), Err(Error::EndOfInput));
		}

		#[test]
		fn success_simple() {
			let double_parser = is('2').and(is('3'));
			let mut input = Input::new_from_chars("23".chars(), None);
			assert_eq!(double_parser(&mut input), Ok('3'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn success_chained() {
			let triple_parser = is('2').and(is('3')).and(is('4'));
			let mut input = Input::new_from_chars("234".chars(), None);
			assert_eq!(triple_parser(&mut input), Ok('4'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn fail_simple() {
			let double_parser = is('2').and(is('3'));
			let mut input = Input::new_from_chars("22".chars(), None);
			assert_eq!(
				double_parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 2)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn fail_chained() {
			let triple_parser = is('2').and(is('3')).and(is('4'));
			let mut input = Input::new_from_chars("233".chars(), None);
			assert_eq!(
				triple_parser(&mut input),
				Err(Error::UnexpectedToken(None, Position::new(1, 3)))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}
}
