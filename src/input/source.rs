use crate::input::core::InputToken;

/// A low-level trait representing a source of input tokens.
///
/// `InputSource` is the bridge between a raw token iterator and the [`crate::input::core::Input`]
/// type. Implementors are responsible for producing tokens one at a time, supporting
/// non-destructive peeking, and reporting an optional source name (e.g., a file path) used in
/// error messages.
///
/// This trait is not intended to be used directly by parser authors. Instead, use
/// [`crate::input::core::Input`], which wraps an `InputSource` and exposes the full parser
/// interface including lookahead and position tracking.
///
/// Two built-in implementations are available:
/// - [`crate::input::string::StringSource`]: used for string-based parsing, it wraps a `char`
///   iterator.
/// - [`crate::input::token::TokenInputSource`]: used for token-based parsing, regardless of what
///   the underlying token type is (as long as it implements [`InputToken`]).
pub trait InputSource {
	type Token: InputToken;

	/// Returns the name of the underlying source, if any (e.g., a file path).
	///
	/// The source name is included in [`crate::error::Error::UnexpectedToken`] to help
	/// identify where in the input a parse failure occurred.
	fn source_name(&self) -> Option<&String>;

	/// Consumes and returns the next token from the source, or `None` if the source is
	/// exhausted.
	fn next_token(&mut self) -> Option<Self::Token>;

	/// Returns a reference to the next token without consuming it, or `None` if the source
	/// is exhausted. Repeated calls must return the same token.
	fn peek(&mut self) -> Option<&Self::Token>;
}
