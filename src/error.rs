//! Error types returned by parsers when they fail to match input.
//!
//! This module defines [`Error`], the single error type used throughout the crate. Every parser
//! returns `Result<Output, Error>`, so understanding the variants is enough to handle all failure
//! cases.

use crate::input::Position;
use std::fmt::{Debug, Display};

pub trait MismatchElement: Display + Debug {}

impl<T> MismatchElement for T where T: Display + Debug {}

impl PartialEq for dyn MismatchElement {
	fn eq(&self, other: &Self) -> bool {
		self.to_string() == other.to_string()
	}
}

#[derive(PartialEq, Debug)]
pub struct Mismatch {
	expected: Option<Box<dyn MismatchElement>>,
	found: Option<Box<dyn MismatchElement>>,
}

impl Mismatch {
	pub fn new<E1, E2>(expected: E1, found: E2) -> Mismatch
	where
		E1: MismatchElement + 'static,
		E2: MismatchElement + 'static,
	{
		let expected = Box::new(expected);
		let found = Box::new(found);
		Mismatch {
			expected: Some(expected),
			found: Some(found),
		}
	}

	pub fn without_found<E>(expected: E) -> Mismatch
	where
		E: MismatchElement + 'static,
	{
		let expected = Box::new(expected);
		Mismatch {
			expected: Some(expected),
			found: None,
		}
	}

	pub fn without_expectation<E>(found: E) -> Mismatch
	where
		E: MismatchElement + 'static,
	{
		let found = Box::new(found);
		Mismatch {
			expected: None,
			found: Some(found),
		}
	}

	pub fn replace_expectation<E>(&mut self, expected: E)
	where
		E: MismatchElement + 'static,
	{
		self.expected.replace(Box::new(expected));
	}
}

impl Display for Mismatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.expected {
			Some(ref expected) => match self.found {
				Some(ref found) => write!(f, "Expected: {expected}, found: {found}"),
				None => write!(f, "Expected: {expected}"),
			},
			None => match self.found {
				Some(ref found) => write!(f, "Found: {found}"),
				None => panic!("Invalid error mismatch."),
			},
		}
	}
}

/// The error type returned by all parsers in this crate.
///
/// A parser returns `Err(Error::UnexpectedToken)` when the next token does not satisfy its
/// requirements, and `Err(Error::EndOfInput)` when it needs more tokens but the input stream is
/// exhausted.
///
/// # Examples
///
/// ```
/// use yapcol::input::Position;
/// use yapcol::{Error, Input, Mismatch, any, is};
///
/// let tokens = vec!['a'];
/// let source_name = Some(String::from("file.txt"));
/// let mut input = Input::new_from_chars(tokens, source_name.clone());
///
/// // Fails with UnexpectedToken when the token does not match.
/// let output = is('b')(&mut input);
/// let mismatch = Mismatch::new('b', 'a');
/// assert_eq!(
/// 	output,
/// 	Err(Error::UnexpectedToken(
/// 		source_name,
/// 		Position::new(1, 1),
/// 		Some(mismatch)
/// 	))
/// );
///
/// // Fails with EndOfInput when the stream is exhausted.
/// is('a')(&mut input).unwrap(); // Consume the only token
/// assert_eq!(any()(&mut input), Err(Error::EndOfInput(None)));
/// ```
#[derive(Debug, PartialEq)]
pub enum Error {
	/// The next token was present but did not satisfy the parser's requirements.
	///
	/// The first field is the optional source name (e.g., a file name). The second is the
	/// position (in the input source) where the unexpected token was found. The third field is an
	/// optional mismatch, detailing what was found and what was expected.
	UnexpectedToken(Option<String>, Position, Option<Mismatch>),
	/// The input stream was exhausted before the parser could match.
	///
	/// It contains an optional mismatch element, describing what was expected.
	EndOfInput(Option<Box<dyn MismatchElement>>),
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::UnexpectedToken(Some(source_name), pos, None) => {
				write!(f, "Unexpected token at {}:{}.", source_name, pos)
			}
			Error::UnexpectedToken(Some(source_name), pos, Some(mismatch)) => {
				write!(f, "Unexpected token at {source_name}:{pos}. {mismatch}")
			}
			Error::UnexpectedToken(None, pos, None) => write!(f, "Unexpected token at {pos}."),
			Error::UnexpectedToken(None, pos, Some(mismatch)) => {
				write!(f, "Unexpected token at {pos}. {mismatch}")
			}
			Error::EndOfInput(Some(expected)) => {
				write!(f, "End of input reached when expected {}.", expected)
			}
			Error::EndOfInput(None) => write!(f, "End of input reached."),
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
			let m1 = Mismatch::without_expectation("hello");
			let m2 = Mismatch::without_expectation("hello");
			assert_eq!(m1, m2);
		}

		#[test]
		fn different_expectation_presence() {
			let m1 = Mismatch::new("hello", "p");
			let m2 = Mismatch::without_expectation("hello");
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
