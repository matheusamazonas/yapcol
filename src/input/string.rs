use crate::input::{InputSource, InputToken, Position};
use std::iter::Peekable;

#[derive(Clone, PartialEq, Debug)]
pub struct CharToken {
	token: char,
	position: Position,
}

impl InputToken for CharToken {
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
	fn new(token: char, position: Position) -> Self {
		Self { token, position }
	}
}

pub(crate) struct StringInputSource<I>
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

impl<I> StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	pub(crate) fn new<S>(source: S) -> Self
	where
		S: IntoIterator<Item = char, IntoIter = I>,
	{
		StringInputSource {
			source: source.into_iter().peekable(),
			source_name: String::from("input"),
			position: Position::new(1, 1),
			peeked_token: None,
		}
	}
}
