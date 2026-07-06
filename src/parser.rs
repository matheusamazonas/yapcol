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

use crate::Mismatch;
use crate::combinators::*;
use crate::error::{Error, MismatchElement};
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
	/// Overrides the expectation in the potential error returned by the current parser in the case
	/// of failure.
	///
	/// When a parser fails, the error may contain information about what was expected. This method
	/// replaces that expectation with the provided value, which is useful for producing clearer
	/// error messages.
	///
	/// # Parameters
	/// - `self`: The current parser.
	/// - `expectation`: The value to use as the new expectation in any error produced by the
	///   current parser.
	///
	/// # Returns
	/// A new parser that behaves identically to the current one on success, but replaces the
	/// expectation in any error with `expectation`.
	///
	/// # Examples
	/// ```rust
	/// use yapcol::{Input, Parser, is};
	///
	/// let parser = is('A').with_expectation("uppercase A");
	///
	/// let mut input = Input::new_from_chars("B".chars(), None);
	/// let error = parser(&mut input).unwrap_err();
	/// assert!(error.to_string().contains("uppercase A"));
	/// ```
	///
	/// # Errors
	/// If the current parser fails, the error is returned with its expectation replaced by
	/// `expectation`.
	fn with_expectation<E>(self, expectation: E) -> impl Parser<IT, O>
	where
		E: MismatchElement + Clone + 'static,
		Self: Sized,
	{
		move |input| match self(input) {
			Ok(result) => Ok(result),
			Err(Error::EndOfInput(Some(_))) => {
				// Replace the expectation.
				Err(Error::EndOfInput(Some(Box::new(expectation.clone()))))
			}
			Err(Error::EndOfInput(None)) => {
				Err(Error::EndOfInput(Some(Box::new(expectation.clone()))))
			}
			Err(Error::UnexpectedToken(s, p, Some(mut mismatch))) => {
				mismatch.replace_expectation(expectation.clone());
				Err(Error::UnexpectedToken(s, p, Some(mismatch)))
			}
			Err(Error::UnexpectedToken(s, p, None)) => {
				let mismatch = Mismatch::without_found(expectation.clone());
				Err(Error::UnexpectedToken(s, p, Some(mismatch)))
			}
			Err(error) => Err(error),
		}
	}

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
	/// caring about its value and then continue parsing with a second parser.
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

	/// A shortcut for the [`end_of_input`] combinator applied after this parser.
	///
	/// Runs this parser and then asserts that the entire input has been consumed. Fails if any
	/// tokens remain in the input after this parser succeeds.
	///
	/// # Examples
	///
	/// ```
	/// use yapcol::{Error, Input, Parser, any};
	///
	/// let mut input = Input::new_from_chars("a".chars(), None);
	/// assert!(any().exhaustive()(&mut input).is_ok());
	///
	/// let mut input = Input::new_from_chars("ab".chars(), None);
	/// assert!(any().exhaustive()(&mut input).is_err()); // 'b' was not consumed.
	/// ```
	fn exhaustive(self) -> impl Parser<IT, O>
	where
		Self: Sized,
	{
		move |input| {
			let output = self(input)?;
			end_of_input()(input)?;
			Ok(output)
		}
	}

	/// Runs the current parser and discards its output, returning `()` on success.
	///
	/// This is a shortcut for `.map(|_| ())`. It is useful when the output of a parser is not
	/// needed, but its side effect (consuming input) is.
	///
	/// # Parameters
	/// - `self`: The current parser whose output is discarded on success.
	///
	/// # Returns
	/// A new parser that runs the current parser, but returns `()` instead of the original output.
	///
	/// # Examples
	/// ```rust
	/// use yapcol::{Input, Parser, any};
	///
	/// let mut input = Input::new_from_chars("a".chars(), None);
	/// let parser = any().discard();
	/// assert_eq!(parser(&mut input), Ok(()));
	/// ```
	///
	/// # Errors
	/// If the current parser fails, its error is returned.
	fn discard(self) -> impl Parser<IT, ()>
	where
		Self: Sized,
	{
		self.map(|_| ())
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

	/// A shortcut for the [`many`] combinator.
	fn many(self) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| many(&self)(input)
	}

	/// A shortcut for the [`many_collect`] combinator.
	fn many_collect(self) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| many_collect(&self)(input)
	}

	/// A shortcut for the [`many_until`] combinator.
	fn many_until<PE, OE>(self, end: &PE) -> impl Parser<IT, usize>
	where
		Self: Sized,
		PE: Parser<IT, OE>,
	{
		move |input| many_until(&self, end)(input)
	}

	/// A shortcut for the [`many_until_collect`] combinator.
	fn many_until_collect<PE, OE>(self, end: &PE) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
		PE: Parser<IT, OE>,
	{
		move |input| many_until_collect(&self, end)(input)
	}

	/// A shortcut for the [`once_or_more`] combinator.
	fn once_or_more(self) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| once_or_more(&self)(input)
	}

	/// A shortcut for the [`once_or_more_collect`] combinator.
	fn once_or_more_collect(self) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| once_or_more_collect(&self)(input)
	}

	/// A shortcut for the [`up_to`] combinator.
	fn up_to(self, max_count: usize) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| up_to(&self, max_count)(input)
	}

	// A shortcut for the [`up_to_collect`] combinator
	fn up_to_collect(self, max_count: usize) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| up_to_collect(&self, max_count)(input)
	}

	/// A shortcut for the [`once_up_to`] combinator.
	fn once_up_to(self, max_count: usize) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| once_up_to(&self, max_count)(input)
	}

	// A shortcut for the [`once_up_to_collect`] combinator
	fn once_up_to_collect(self, max_count: usize) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| once_up_to_collect(&self, max_count)(input)
	}

	/// A shortcut for the [`at_least`] combinator.
	fn at_least(self, min_count: usize) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| at_least(&self, min_count)(input)
	}

	/// A shortcut for the [`at_least_collect`] combinator.
	fn at_least_collect(self, min_count: usize) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| at_least_collect(&self, min_count)(input)
	}

	/// A shortcut for the [`count`] combinator.
	fn count(self, c: usize) -> impl Parser<IT, usize>
	where
		Self: Sized,
	{
		move |input| count(&self, c)(input)
	}

	/// A shortcut for the [`count_collect`] combinator.
	fn count_collect(self, c: usize) -> impl Parser<IT, Vec<O>>
	where
		Self: Sized,
	{
		move |input| count_collect(&self, c)(input)
	}

	/// A shortcut for the [`between`] combinator where the callee parser is the one in between.
	fn between<PO, PC, OO, OC>(self, open: &PO, close: &PC) -> impl Parser<IT, O>
	where
		PO: Parser<IT, OO>,
		PC: Parser<IT, OC>,
		Self: Sized,
	{
		move |input| between(open, &self, close)(input)
	}
}

impl<IT, O, X> Parser<IT, O> for X
where
	X: Fn(&mut Input<IT>) -> Result<O, Error>,
	IT: InputToken,
{
}

/// A convenience alias for [`Parser`] specialized to character-stream input.
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

	mod exhaustive {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('a').exhaustive();
			let mut input = Input::new_from_chars("".chars(), None);
			let expected = Box::new("a");
			assert_eq!(parser(&mut input), Err(Error::EndOfInput(Some(expected))));
		}

		#[test]
		fn leftover() {
			let parser = is('a').exhaustive();
			let mut input = Input::new_from_chars("aa".chars(), None);
			let position = Position::new(1, 2);
			let mismatch = Mismatch::new("end of input", "a");
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(None, position, Some(mismatch)))
			);
		}

		#[test]
		fn success_simple() {
			let parser = is('a').exhaustive();
			let mut input = Input::new_from_chars("a".chars(), None);
			assert_eq!(parser(&mut input), Ok('a'));
		}

		#[test]
		fn success_separated_by_1() {
			let is_digit = |c: &char| c.to_digit(10);
			let parse_number = satisfy(is_digit);
			let parse_comma = is(',');
			let parser = separated_by1(&parse_number, &parse_comma);
			let parser = parser.exhaustive();
			let mut input = Input::new_from_chars("1,1,1,1,1".chars(), None);
			assert_eq!(parser(&mut input), Ok(vec![1, 1, 1, 1, 1]));
		}

		#[test]
		fn fail_separated_by_1() {
			let is_digit = |c: &char| c.to_digit(10);
			let parse_number = satisfy(is_digit);
			let parse_comma = is(',');
			let parser = separated_by1(&parse_number, &parse_comma);
			let parser = parser.exhaustive();
			let mut input = Input::new_from_chars("1,1,1,1,1#".chars(), None);
			let position = Position::new(1, 10);
			let mismatch = Mismatch::new("end of input", '#');
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(None, position, Some(mismatch)))
			);
		}
	}

	mod discard {
		use crate::*;

		#[test]
		fn success() {
			let parser = is('a').discard();
			let mut input = Input::new_from_chars("a".chars(), None);
			assert_eq!(parser(&mut input).unwrap(), ());
		}

		#[test]
		fn fail() {
			let parser = is('a').discard();
			let mut input = Input::new_from_chars("b".chars(), None);
			let output = parser(&mut input);
			assert!(output.is_err());
		}
	}
}
