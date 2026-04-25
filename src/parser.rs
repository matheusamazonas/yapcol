//! Core parser trait and string parser alias.
//!
//! This module defines the [`Parser`] trait, which is the central abstraction of the `yapcol`
//! crate. A [`Parser`] is any function (or closure) that takes a mutable reference to an
//! [`Input`] and returns a `Result` containing either a successfully parsed value or an
//! [`Error`].
//!
//! The trait is automatically implemented for any function with the signature
//! `Fn(&mut Input<IT>) -> Result<Output, Error>`, so you can use plain functions or closures as
//! parsers without any boilerplate.
//!
//! # Combinators
//!
//! The [`Parser`] trait exposes some built-in utility methods that let you transform and chain
//! parsers:
//!
//! - [`map`](Parser::map): transforms the output of a parser with a function.
//! - [`and_then`](Parser::and_then): chains two parsers, where the second depends on the output
//!   of the first.
//! - [`and`](Parser::and): sequences two parsers, discarding the output of the first.
//!
//! # String Parsing
//!
//! For the common case of parsing character streams, this module also provides the [`StringParser`]
//! convenience trait, which is a specialization of [`Parser`] for char-based input. It is
//! automatically implemented for any function `Fn(&mut StringInput) -> Result<Output, Error>`.

use crate::combinators::*;
use crate::error::Error;
use crate::input::{CharToken, Input, InputToken, StringInput};

/// The core trait of the `yapcol` crate, representing a parser.
///
/// A `Parser` is a function (or any type that implements `Fn`) that takes a mutable reference
/// to an [`Input`] and returns a `Result` containing either the successfully parsed output
/// of type `O` or an [`Error`].
///
/// This trait is automatically implemented for any function with the signature
/// `Fn(&mut Input<I>) -> Result<Output, Error>`.
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
/// use yapcol::{Error, Input, StringInput, is};
///
/// fn my_uppercase_parser(input: &mut StringInput) -> Result<char, Error> {
/// 	// You can use existing parsers inside your custom parser
/// 	is('A')(input)
/// }
///
/// let mut input = Input::new_from_chars("Abc".chars(), None);
/// assert_eq!(my_uppercase_parser(&mut input), Ok('A'));
/// ```
///
/// Most of the time, you will use the built-in combinators which return `impl Parser`:
///
/// ```
/// use yapcol::{Input, is};
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
	/// use yapcol::{Input, Parser, any, satisfy};
	///
	/// let is_digit = |c: &char| if c.is_ascii_digit() { Some(*c) } else { None };
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
	/// use yapcol::{Input, Parser, is};
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
	/// use yapcol::{Input, Parser, any, is};
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

	/// A shortcut for the [`attempt`] combinator.
	fn attempt(self) -> impl Parser<IT, O>
	where
		Self: Sized,
	{
		move |input| attempt(&self)(input)
	}

	/// A shortcut for the [`maybe`] combinator.
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
/// for any function `Fn(&mut StringInput) -> Result<Output, Error>`. It exists purely to reduce
/// type-annotation noise when working with string-based parsers.
pub trait StringParser<O>: Parser<CharToken, O> {}

impl<O, X> StringParser<O> for X where X: Fn(&mut StringInput) -> Result<O, Error> {}

#[cfg(test)]
mod tests {
	mod map {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('2').map(|c: char| c.to_digit(10));
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(
				parser(&mut input),
				Err(Error::EndOfInput(Some(Box::new('2'))))
			);
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
			let mismatch = Mismatch::new('2', '3');
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
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
			let mismatch = Mismatch::new('5', '3');
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}

	mod and_then {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let double_parser = is('2').and_then(is);
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(
				double_parser(&mut input),
				Err(Error::EndOfInput(Some(Box::new('2'))))
			);
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
			let mismatch = Mismatch::new('2', '3');
			assert_eq!(
				double_parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 2),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn fail_chained() {
			let triple_parser = is('2').and_then(is).and_then(is);
			let mut input = Input::new_from_chars("223".chars(), None);
			let mismatch = Mismatch::new('2', '3');
			assert_eq!(
				triple_parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 3),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}

	mod and {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let double_parser = is('2').and(is('3'));
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(
				double_parser(&mut input),
				Err(Error::EndOfInput(Some(Box::new('2'))))
			);
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
			let mismatch = Mismatch::new('3', '2');
			assert_eq!(
				double_parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 2),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn fail_chained() {
			let triple_parser = is('2').and(is('3')).and(is('4'));
			let mut input = Input::new_from_chars("233".chars(), None);
			let mismatch = Mismatch::new('4', '3');
			assert_eq!(
				triple_parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 3),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}
}
