use crate::input::{InputSource, InputToken};
use std::iter::Peekable;

pub struct TokenInputSource<I>
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

impl<I> TokenInputSource<I>
where
	I: Iterator<Item: InputToken>,
{
	pub fn new<S>(source: S) -> Self
	where
		S: IntoIterator<Item: InputToken, IntoIter = I>,
	{
		Self {
			source_name: String::from("test"),
			stream: source.into_iter().peekable(),
		}
	}
}
