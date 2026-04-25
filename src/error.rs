//! Error types returned by parsers when they fail to match input.
//!
//! This module defines [`Error`], the single error type used throughout the crate. Every parser
//! returns `Result<Output, Error>`, so understanding the variants is enough to handle all failure
//! cases.

use crate::input::Position;
use std::fmt::{Debug, Display};

pub struct Mismatch {
	expected: Box<dyn Display>,
	found: Box<dyn Display>,
}

impl Mismatch {
	pub fn new<T>(expected: T, found: T) -> Mismatch
	where
		T: Display + Clone + 'static,
	{
		let expected = Box::new(expected.clone());
		let found = Box::new(found.clone());
		Mismatch { expected, found }
	}
}

impl Display for Mismatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Expected: {}, found: {}", self.expected, self.found)
	}
}

impl Debug for Mismatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(self, f)
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
/// assert_eq!(any()(&mut input), Err(Error::EndOfInput));
/// ```
#[derive(Debug)]
pub enum Error {
	/// The next token was present but did not satisfy the parser's requirements.
	///
	/// The first field is the optional source name (e.g. a file name), and the second is the
	/// position (in the input source) where the unexpected token was found.
	UnexpectedToken(Option<String>, Position, Option<Mismatch>),
	/// The input stream was exhausted before the parser could match.
	EndOfInput,
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Error::EndOfInput, Error::EndOfInput) => true,
			(Error::UnexpectedToken(s1, p1, e1), Error::UnexpectedToken(s2, p2, e2)) => {
				match (e1, e2) {
					(Some(e1), Some(e2)) => {
						s1 == s2 && p1 == p2 && e1.to_string() == e2.to_string()
					}
					(None, None) => true,
					_ => false,
				}
			}
			_ => false,
		}
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::UnexpectedToken(Some(source_name), pos, None) => {
				write!(f, "Unexpected token at {}:{}.", source_name, pos)
			}
			Error::UnexpectedToken(Some(source_name), pos, Some(expectation)) => {
				write!(
					f,
					"Unexpected token at {source_name}:{pos}. Expected: {}, found: {}",
					expectation.expected, expectation.found
				)
			}
			Error::UnexpectedToken(None, pos, _) => write!(f, "Unexpected token at {}.", pos),
			Error::EndOfInput => write!(f, "End of input reached."),
		}
	}
}
