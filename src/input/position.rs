use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
	line: usize,
	column: usize,
}

impl Position {
	pub fn new(line: usize, column: usize) -> Position {
		Position { line, column }
	}

	pub fn advance_column(&mut self) {
		self.column += 1;
	}
}

impl Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}", self.line, self.column)
	}
}
