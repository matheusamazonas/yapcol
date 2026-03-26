use crate::input::{Input, InputSource, Position, PositionToken, TokenLocation};
use std::collections::VecDeque;
use std::iter::Peekable;

#[derive(Clone, PartialEq, Debug)]
pub struct CharToken {
	token: char,
	position: Position,
}

impl PositionToken for CharToken {
	type Token = char;

	fn token(&self) -> &char {
		&self.token
	}

	fn token_owned(self) -> char {
		self.token
	}

	fn position(&self) -> Position {
		self.position
	}
}

impl CharToken {
	pub fn new(token: char, position: Position) -> Self {
		Self { token, position }
	}
}

struct StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	source: Peekable<I>,
	source_name: String,
	position: Position,
	peeked_token: Option<CharToken>,
}

impl<I> InputSource for StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	type Token = CharToken;

	fn source_name(&self) -> String {
		self.source_name.clone()
	}

	fn next_token(&mut self) -> Option<CharToken> {
		let token = self.source.next()?;
		let token = CharToken::new(token, self.position);
		self.position.advance_column();
		Some(token)
	}

	fn peek(&mut self) -> Option<&CharToken> {
		let token = self.source.peek()?;
		self.peeked_token = Some(CharToken::new(*token, self.position));
		self.peeked_token.as_ref()
	}
}

pub fn new_string_input<'a, S, I>(source: S) -> Input<'a, CharToken>
where
	S: IntoIterator<Item = char, IntoIter = I>,
	I: Iterator<Item = char> + 'a,
{
	let source = StringInputSource {
		source: source.into_iter().peekable(),
		source_name: String::from("input"),
		position: Position::new(1, 1),
		peeked_token: None,
	};

	Input {
		source: Box::new(source),
		consumed_count: 0,
		next_location: TokenLocation::Stream,
		look_ahead_frames: Vec::new(),
		look_ahead_buffer: VecDeque::new(),
		last_token_position: Position::new(1, 1),
	}
}
