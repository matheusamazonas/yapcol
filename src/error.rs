//! Error types returned by parsers when they fail to match input.
//!
//! This module defines [`Error`], the single error type used throughout the crate. Every parser
//! returns `Result<Output, Error>`, so understanding the variants is enough to handle all failure
//! cases.

use crate::input::Position;
use std::fmt::{Debug, Display};

pub trait MismatchElement: Display + Debug {}

impl<T> MismatchElement for T where T: Display + Debug {}

pub struct Mismatch {
	expected: Box<dyn MismatchElement>,
	found: Box<dyn MismatchElement>,
}

impl Mismatch {
	pub fn new<E1, E2>(expected: E1, found: E2) -> Mismatch
	where
		E1: MismatchElement + Clone + 'static,
		E2: MismatchElement + Clone + 'static,
	{
		let expected = Box::new(expected.clone());
		let found = Box::new(found.clone());
		Mismatch { expected, found }
	}

	pub fn same(om1: &Option<Mismatch>, om2: &Option<Mismatch>) -> bool {
		match (om1, om2) {
			(None, None) => true,
			(Some(m1), Some(m2)) => m1.expected.to_string() == m2.expected.to_string(),
			_ => false,
		}
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
/// assert_eq!(any()(&mut input), Err(Error::EndOfInput(None)));
/// ```
#[derive(Debug)]
pub enum Error {
	/// The next token was present but did not satisfy the parser's requirements.
	///
	/// The first field is the optional source name (e.g. a file name), and the second is the
	/// position (in the input source) where the unexpected token was found.
	UnexpectedToken(Option<String>, Position, Option<Mismatch>),
	/// The input stream was exhausted before the parser could match.
	EndOfInput(Option<Box<dyn MismatchElement>>),
}

impl PartialEq for Error {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Error::EndOfInput(om1), Error::EndOfInput(om2)) => match (om1, om2) {
				(None, None) => true,
				(Some(e1), Some(e2)) => e1.to_string() == e2.to_string(),
				_ => false,
			},
			(Error::UnexpectedToken(s1, p1, om1), Error::UnexpectedToken(s2, p2, om2)) => {
				s1 == s2 && p1 == p2 && Mismatch::same(om1, om2)
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
			Error::UnexpectedToken(Some(source_name), pos, Some(mismatch)) => {
				write!(
					f,
					"Unexpected token at {source_name}:{pos}. Expected: {}, found: {}",
					mismatch.expected, mismatch.found
				)
			}
			Error::UnexpectedToken(None, pos, None) => write!(f, "Unexpected token at {pos}."),
			Error::UnexpectedToken(None, pos, Some(mismatch)) => {
				write!(
					f,
					"Unexpected token at {pos}. Expected: {}, found: {}",
					mismatch.expected, mismatch.found
				)
			}
			Error::EndOfInput(Some(expected)) => {
				write!(f, "End of input reached when expected {}.", expected)
			}
			Error::EndOfInput(None) => write!(f, "End of input reached."),
		}
	}
}
