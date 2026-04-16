use crate::input::core::{Input, InputToken};
use crate::input::position::Position;
use crate::input::source::InputSource;
use std::iter::Peekable;

/// A token produced when parsing a string (character stream) input.
///
/// `CharToken` wraps a single `char` together with its [`Position`] in the source, so that
/// parsers can report accurate error position when a parsing error occurs.
///
/// `CharToken` is the [`InputToken`] implementation used by [`StringInput`], and is the token type
/// for all string-based parsers.
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

/// The [`InputSource`] implementation that drives [`StringInput`].
///
/// `StringInputSource` wraps a peekable `char` iterator and tracks the current [`Position`]
/// (line and column) as characters are consumed, producing [`CharToken`]s for each input character.
///
/// This struct is an internal implementation detail; use [`StringInput::new_from_chars`] to
/// create a string-based input.
struct StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	source: Peekable<I>,
	source_name: Option<String>,
	position: Position,
	peeked_token: Option<CharToken>,
}

impl<I> StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	pub(crate) fn new<S>(source: S, source_name: Option<String>) -> Self
	where
		S: IntoIterator<Item = char, IntoIter = I>,
	{
		StringInputSource {
			source: source.into_iter().peekable(),
			source_name,
			position: Position::new(1, 1),
			peeked_token: None,
		}
	}
}

impl<I> InputSource for StringInputSource<I>
where
	I: Iterator<Item = char>,
{
	type Token = CharToken;

	fn source_name(&self) -> Option<&String> {
		self.source_name.as_ref()
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

/// A type alias for [`Input`] specialized to character streams.
///
/// `StringInput` is the concrete input type used when parsing text. It is constructed via
/// [`StringInput::new_from_chars`] and tracks the current [`Position`] (line and column) as
/// characters are consumed.
///
/// # Examples
///
/// ```
/// use yapcol::input::string::StringInput;
/// let mut input = StringInput::new_from_chars("hello".chars(), Some("input.md".to_string()));
/// ```
pub type StringInput<'a> = Input<'a, CharToken>;

impl<'a> StringInput<'a> {
	/// Creates a new `StringInput` from any iterator of `char`s.
	///
	/// # Arguments
	///
	/// - `chars`: Any value that can be turned into a `char` iterator, such as [`str::chars`].
	/// - `source_name`: An optional name for the source (e.g. a file path), to be included in
	///   error messages.
	///
	/// # Examples
	///
	/// ```
	/// use yapcol::input::string::StringInput;
	/// let mut input = StringInput::new_from_chars("hello".chars(), Some(String::from("in.txt")));
	/// ```
	pub fn new_from_chars<S>(chars: S, source_name: Option<String>) -> Input<'a, CharToken>
	where
		S: IntoIterator<Item = char, IntoIter: Iterator<Item = char> + 'a>,
	{
		let source = StringInputSource::new(chars, source_name);
		Input::new(Box::new(source))
	}
}
