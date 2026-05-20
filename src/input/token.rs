use crate::input::core::InputToken;
use crate::input::source::InputSource;
use std::iter::Peekable;

pub(crate) struct TokenInputSource<I>
where
	I: Iterator<Item: InputToken>,
{
	source_name: Option<String>,
	stream: Peekable<I>,
}

impl<I> InputSource for TokenInputSource<I>
where
	I: Iterator<Item: InputToken>,
{
	type Token = I::Item;

	fn source_name(&self) -> Option<&String> {
		self.source_name.as_ref()
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
	pub fn new<S>(source: S, source_name: Option<String>) -> Self
	where
		S: IntoIterator<Item: InputToken, IntoIter = I>,
	{
		Self {
			source_name,
			stream: source.into_iter().peekable(),
		}
	}
}
