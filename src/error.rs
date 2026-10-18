//! Error types returned by parsers when they fail to match input.
//!
//! This module defines [`Error`], the single error type used throughout the crate. Every parser
//! returns `Result<Output, Error>`, so understanding the variants is enough to handle all failure
//! cases.

use crate::input::Position;
use std::fmt::{Debug, Display};
use std::rc::Rc;

/// A trait for types that can be used as elements in a [`Mismatch`].
///
/// Any type that implements both [`Display`] and [`Debug`] automatically implements this trait.
pub trait MismatchElement: Display + Debug {}

impl<T> MismatchElement for T where T: Display + Debug {}

impl PartialEq for dyn MismatchElement {
	fn eq(&self, other: &Self) -> bool {
		self.to_string() == other.to_string()
	}
}

/// Describes a mismatch between what was expected and what was found during parsing.
///
/// A `Mismatch` is carried by [`Error::UnexpectedToken`] to provide human-readable context about
/// why a parser failed. There are two variations: one used whenever the expected and the found
/// values are known [`Mismatch::ExpectedAndFound`], and one used whenever the expected value is not
/// known [`Mismatch::Found`].
#[derive(PartialEq, Debug)]
pub enum Mismatch {
	ExpectedAndFound {
		expected: Box<dyn MismatchElement>,
		found: Box<dyn MismatchElement>,
	},
	Found(Box<dyn MismatchElement>),
}

impl Mismatch {
	/// Creates a `Mismatch` with both an expected value and a found value.
	pub fn new<E1, E2>(expected: E1, found: E2) -> Mismatch
	where
		E1: MismatchElement + 'static,
		E2: MismatchElement + 'static,
	{
		let expected = Box::new(expected);
		let found = Box::new(found);
		Mismatch::ExpectedAndFound { expected, found }
	}

	/// Creates a `Mismatch` with only a found value when no expectation is available.
	pub fn new_without_expected<E>(found: E) -> Mismatch
	where
		E: MismatchElement + 'static,
	{
		let found = Box::new(found);
		Mismatch::Found(found)
	}

	/// Replaces the expected value of this `Mismatch`, overwriting any previously set expectation.
	pub fn replace_expectation<E>(self, new_expected: E) -> Self
	where
		E: MismatchElement + 'static,
	{
		match self {
			Mismatch::ExpectedAndFound { found, .. } | Mismatch::Found(found) => {
				Mismatch::ExpectedAndFound {
					expected: Box::new(new_expected),
					found,
				}
			}
		}
	}
}

impl Display for Mismatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Mismatch::ExpectedAndFound { expected, found } => {
				write!(f, "Expected: {expected}, found: {found}")
			}
			Mismatch::Found(found) => {
				write!(f, "Found: {found}")
			}
		}
	}
}

/// The error type returned by all parsers in this crate.
///
/// # Examples
///
/// ```
/// use yapcol::input::Position;
/// use yapcol::{Error, Input, Mismatch, any, is};
///
/// let tokens = vec!['a'];
/// let mut input = Input::new_from_chars(tokens, String::from("file.txt"));
///
/// // Fails with UnexpectedToken when the token does not match.
/// let output = is('b')(&mut input);
/// let mismatch = Mismatch::new('b', 'a');
/// assert_eq!(
/// 	output,
/// 	Err(Error::UnexpectedToken(
/// 		input.source_name(),
/// 		Position::new(1, 1),
/// 		Some(mismatch)
/// 	))
/// );
///
/// // Fails with EndOfInput when the stream is exhausted.
/// is('a')(&mut input).unwrap(); // Consume the only token
/// assert_eq!(
/// 	any()(&mut input),
/// 	Err(Error::EndOfInput(input.source_name(), None))
/// );
/// ```
#[derive(Debug, PartialEq)]
pub enum Error {
	/// The next token was present but did not satisfy the parser's requirements.
	///
	/// The first field is the source name (e.g., a file name). The second is the position (in the
	/// input source) where the unexpected token was found. The third field is an optional mismatch,
	/// detailing what was found and what was expected.
	UnexpectedToken(Rc<String>, Position, Option<Mismatch>),
	/// The input stream was exhausted before the parser could match.
	///
	/// The first field is the source name (e.g., a file name). The second one is an optional
	/// mismatch element, describing what was expected.
	EndOfInput(Rc<String>, Option<Box<dyn MismatchElement>>),
	/// A repetition parser (e.g., [`crate::many`], [`crate::once_or_more`] or
	/// [`crate::many_until_collect`]) detected that the inner parser succeeded without consuming
	/// any input, which would cause an infinite loop.
	///
	/// The first field is the source name (e.g., a file name). The second field is the position in
	/// the input where the loop was detected.
	///
	/// A parser is said to be *non-consuming* when it matches successfully without advancing the
	/// input position. When such a parser is used inside a repetition combinator, every iteration
	/// would succeed indefinitely without making progress, so the combinator returns this error
	/// instead of looping forever.
	///
	/// # Examples
	///
	/// ```
	/// use yapcol::input::Position;
	/// use yapcol::{Error, Input, many, success};
	///
	/// // `success` always succeeds without consuming any input, so `many` detects the loop.
	/// let parser = success(());
	/// let mut input = Input::new_from_chars("abc".chars(), String::from("file.txt"));
	/// assert_eq!(
	/// 	many(&parser)(&mut input),
	/// 	Err(Error::NonConsumingLoop(
	/// 		input.source_name(),
	/// 		Position::new(1, 1)
	/// 	))
	/// );
	/// ```
	NonConsumingLoop(Rc<String>, Position),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::UnexpectedToken(source_name, pos, None) => {
				write!(f, "Unexpected token at {}:{}.", source_name, pos)
			}
			Error::UnexpectedToken(source_name, pos, Some(mismatch)) => {
				write!(f, "Unexpected token at {source_name}:{pos}. {mismatch}")
			}
			Error::EndOfInput(source_name, Some(expected)) => {
				write!(
					f,
					"End of input reached at {source_name} when expected {expected}."
				)
			}
			Error::EndOfInput(source_name, None) => {
				write!(f, "End of input reached at {source_name}.")
			}
			Error::NonConsumingLoop(source_name, pos) => {
				write!(f, "Non-consuming parser loop at {source_name}:{pos}.")
			}
		}
	}
}

#[cfg(test)]
mod tests {

	mod mismatch {
		use crate::Mismatch;

		#[test]
		fn same_with_expectation_equal() {
			let m1 = Mismatch::new("h", "p");
			let m2 = Mismatch::new("h", "p");
			assert_eq!(m1, m2);
		}

		#[test]
		fn same_without_expectation_equal() {
			let m1 = Mismatch::new_without_expected("hello");
			let m2 = Mismatch::new_without_expected("hello");
			assert_eq!(m1, m2);
		}

		#[test]
		fn different_expectation_presence() {
			let m1 = Mismatch::new("hello", "p");
			let m2 = Mismatch::new_without_expected("hello");
			assert_ne!(m1, m2);
		}

		#[test]
		fn different_expectation() {
			let m1 = Mismatch::new("hello", "p");
			let m2 = Mismatch::new("hallo", "p");
			assert_ne!(m1, m2);
		}

		#[test]
		fn different_found() {
			let m1 = Mismatch::new("hello", "p");
			let m2 = Mismatch::new("hello", "x");
			assert_ne!(m1, m2);
		}
	}
}
