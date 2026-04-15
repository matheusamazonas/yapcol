use std::fmt::Display;

/// Represents a position in the input source, identified by a line and column number.
///
/// Both line and column numbers are 1-based: the first character of the input is at
/// position `1:1`.
///
/// `Position` is attached to every token produced by the input source and is included in
/// [`crate::error::Error::UnexpectedToken`] to indicate where in the input a parse failure
/// occurred.
///
/// # Examples
///
/// ```
/// use yapcol::input::position::Position;
///
/// let pos = Position::new(1, 1);
/// assert_eq!(pos.to_string(), "1:1");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
	line: usize,
	column: usize,
}

impl Position {
	/// Creates a new `Position` with the given `line` and `column`.
	pub fn new(line: usize, column: usize) -> Position {
		Position { line, column }
	}

	/// Advances the column number by one.
	pub fn advance_column(&mut self) {
		self.column += 1;
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}
