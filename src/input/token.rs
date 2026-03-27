use crate::input::{Input, InputSource, InputToken, Position, TokenLocation};
use std::collections::VecDeque;
use std::iter::Peekable;

struct TokenInputSource<I>
where
	I: Iterator<Item: InputToken>,
{
	source_name: String,
	stream: Peekable<I>,
}

impl<I> InputSource for TokenInputSource<I>
where
	I: Iterator<Item: InputToken>,
{
	type Token = I::Item;

	fn source_name(&self) -> String {
		self.source_name.clone()
	}

	fn next_token(&mut self) -> Option<Self::Token> {
		self.stream.next()
	}

	fn peek(&mut self) -> Option<&Self::Token> {
		self.stream.peek()
	}
}

pub fn new_token_input<'a, S, I>(source: S) -> Input<'a, I::Item>
where
	S: IntoIterator<Item: InputToken, IntoIter = I>,
	I: Iterator<Item: InputToken> + 'a,
{
	let source = TokenInputSource {
		source_name: String::from("test"),
		stream: source.into_iter().peekable(),
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
