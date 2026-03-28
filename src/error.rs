//! Error types returned by parsers when they fail to match input.
//!
//! This module defines [`Error`], the single error type used throughout the crate. Every parser
//! returns `Result<O, Error>`, so understanding the variants is enough to handle all failure
//! cases.

use crate::input::position::Position;
use std::fmt::Display;

/// The error type returned by all parsers in this crate.
///
/// A parser returns `Err(Error::UnexpectedToken)` when the next token does not satisfy its
/// requirements, and `Err(Error::EndOfInput)` when it needs more tokens but the input stream is
/// exhausted.
///
/// # Examples
///
/// ```
/// use yapcol::{is, any};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
/// use yapcol::input::position::Position;
///
/// let tokens = vec!['a'];
/// let mut input = Input::new(tokens);
///
/// // Fails with UnexpectedToken when the token does not match.
/// assert_eq!(is('b')(&mut input), Err(Error::UnexpectedToken(Position::new(1,1))));
///
/// // Fails with EndOfInput when the stream is exhausted.
/// is('a')(&mut input).unwrap(); // Consume the only token
/// assert_eq!(any()(&mut input), Err(Error::EndOfInput));
/// ```
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
	/// The next token was present but did not satisfy the parser's requirements.
	UnexpectedToken(Position),
	/// The input stream was exhausted before the parser could match.
	EndOfInput,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::UnexpectedToken(pos) => write!(f, "Unexpected token at {}.", pos),
			Error::EndOfInput => write!(f, "End of input reached."),
		}
	}
}
