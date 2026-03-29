use crate::input::lookahead::{LookAheadFrame, LookAheadHandler, TokenLocation};
use crate::input::position::Position;
use crate::input::source::InputSource;
use crate::input::token::TokenInputSource;
use std::collections::VecDeque;

pub trait InputToken: Clone {
	type Token: PartialEq + Clone;
	fn token(&self) -> &Self::Token;
	fn token_owned(self) -> Self::Token;
	fn position(&self) -> Position;
}

/// An input stream that can be used to fetch input tokens. It's the most important entity in this
/// module, concentrating all input operations.
pub struct Input<'a, IT> {
	source: Box<dyn InputSource<Token = IT> + 'a>,
	consumed_count: usize,
	next_location: TokenLocation,
	look_ahead_frames: Vec<LookAheadFrame>,
	look_ahead_buffer: VecDeque<IT>,
	last_token_position: Position,
}

impl<'a, IT> Input<'a, IT>
where
	IT: InputToken,
{
	pub(crate) fn new(source: Box<dyn InputSource<Token = IT> + 'a>) -> Self {
		Input {
			source,
			consumed_count: 0,
			next_location: TokenLocation::Stream,
			look_ahead_frames: Vec::new(),
			look_ahead_buffer: VecDeque::new(),
			last_token_position: Position::new(1, 1),
		}
	}

	pub fn new_from_tokens<S, I>(source: S) -> Input<'a, IT>
	where
		S: IntoIterator<Item = IT, IntoIter = I>,
		I: Iterator<Item = IT> + 'a,
	{
		let source = TokenInputSource::new(source);
		Input::new(Box::new(source))
	}

	/// Fetches the next token in the input stream, mutating the input stream.
	pub(crate) fn next_token(&mut self) -> Option<IT> {
		match self.next_location {
			TokenLocation::Stream => {
				self.consumed_count += 1;
				let token = self.source.next_token()?;
				self.last_token_position = token.position();
				Some(token)
			}
			TokenLocation::StreamLookingAhead => {
				let frame = self.look_ahead_frames.last_mut().unwrap();
				match self.source.next_token() {
					None => None,
					Some(token) => {
						self.last_token_position = token.position();
						let cloned = token.clone();
						self.look_ahead_buffer.push_back(token);
						frame.length += 1;
						Some(cloned)
					}
				}
			}
			TokenLocation::BufferHead => {
				self.consumed_count += 1;
				let output = self.look_ahead_buffer.pop_front();
				self.next_location = if self.look_ahead_buffer.is_empty() {
					TokenLocation::Stream
				} else {
					TokenLocation::BufferHead
				};
				let token = output?;
				self.last_token_position = token.position();
				Some(token)
			}
			TokenLocation::BufferTail => {
				let frame = self.look_ahead_frames.last_mut().unwrap();
				let token = self.look_ahead_buffer.get(frame.next_ix()).unwrap();
				frame.length += 1;
				self.next_location = if frame.next_ix() == self.look_ahead_buffer.len() {
					TokenLocation::StreamLookingAhead
				} else {
					TokenLocation::BufferTail
				};
				self.last_token_position = token.position();
				Some(token.clone())
			}
		}
	}

	/// Peeks into the input, returning a reference to the next token. Calling this method does not
	/// mutate the input, and calling it repeatedly will return the same item over and over.
	pub(crate) fn peek(&mut self) -> Option<&IT> {
		match self.next_location {
			TokenLocation::Stream => self.source.peek(),
			TokenLocation::StreamLookingAhead => self.source.peek(),
			TokenLocation::BufferHead => self.look_ahead_buffer.front(),
			TokenLocation::BufferTail => {
				let frame = self.look_ahead_frames.last_mut().unwrap();
				self.look_ahead_buffer.get(frame.next_ix())
			}
		}
	}

	/// How many tokens the input stream has consumed.
	pub(crate) fn consumed_count(&self) -> usize {
		self.consumed_count
	}

	pub fn position(&mut self) -> Position {
		match self.peek() {
			Some(token) => token.position(),
			None => self.last_token_position,
		}
	}

	/// Starts a lookahead operation, putting the input stream into lookahead mode (if it already
	/// wasn't). During lookahead mode, tokens might be fetched, but they won't be consumed. This
	/// implements arbitrary lookahead, where tokens will be cached internally, for as long as the
	/// lookahead mode is enabled.
	/// For more information on disabling lookahead mode, check [`stop_look_ahead`].
	///
	/// This function returns a [`LookAheadHandler`], which *must* be used to stop the look ahead
	/// operation. Failing to use this handler will trigger compilation warnings.
	///
	/// # Nested lookahead operations
	///
	/// [`Input`] supports nested lookahead operations, where each operation is self-contained and
	/// unaware of the others. Repeated calls to this method will create different lookahead
	/// operations, where each one is "deeper" than the previous one.
	///
	/// One rule must be respected when nesting these operations: the order in which the operations
	/// are stopped must be the reverse order of their creation. In order words, only the most
	/// recent (active) lookahead operation might be stopped. The [`LookAheadHandler`]s are there
	/// to enforce this rule.
	pub(crate) fn start_look_ahead(&mut self) -> LookAheadHandler {
		let new_frame = match self.look_ahead_frames.last() {
			Some(previous) => {
				let start_index = previous.start_index + previous.length;
				LookAheadFrame {
					start_index,
					length: 0,
				}
			}
			None => LookAheadFrame {
				start_index: 0,
				length: 0,
			},
		};
		self.next_location = if self.look_ahead_buffer.is_empty()
			|| new_frame.next_ix() == self.look_ahead_buffer.len()
		{
			TokenLocation::StreamLookingAhead
		} else {
			TokenLocation::BufferTail
		};

		let token_id = self.look_ahead_frames.len();
		self.look_ahead_frames.push(new_frame);
		LookAheadHandler::new(token_id)
	}

	/// Stops the current lookahead operation, controlling whether the input stream should
	/// backtrack.
	///
	/// # Arguments
	///
	/// - `handler`: Handler used to stop the lookahead operation. This handler *must* belong to the
	///   latest lookahead operation, enforcing the lookahead rule that only the most recent
	///   operation might be stopped. The function will panic if this invariant is not respected.
	/// - `backtrack`: Whether the input should backtrack. If `true`, all tokens fetched during the
	///   lookahead operation will remain cached, and later fetching will return them. In order
	///   words, it pretends that all the fetching that happened during the lookahead operation
	///   did not happen. If `false`, it discards the tokens fetched during the lookahead
	///   operation, and later fetch requests will return new tokens.
	///
	/// # Nested lookahead operations
	///
	/// A call to this method stops the current lookahead operation but might not stop the
	/// lookahead mode—that will only happen if the operation being stopped is the only active one.
	pub(crate) fn stop_look_ahead(&mut self, handler: LookAheadHandler, backtrack: bool) {
		let frame = self.look_ahead_frames.pop().unwrap();
		if handler.id() != self.look_ahead_frames.len() {
			panic!("Look ahead handler doesn't match current lookahead depth.")
		}

		if !backtrack {
			self.consumed_count += frame.length;
			let buffer_length = self.look_ahead_buffer.len();
			self.look_ahead_buffer
				.truncate(buffer_length - frame.length);
		}

		self.next_location = if self.look_ahead_frames.is_empty() {
			if self.look_ahead_buffer.is_empty() {
				TokenLocation::Stream
			} else {
				TokenLocation::BufferHead
			}
		} else {
			let frame = self.look_ahead_frames.last_mut().unwrap();
			if frame.next_ix() == self.look_ahead_buffer.len() {
				TokenLocation::StreamLookingAhead
			} else {
				TokenLocation::BufferTail
			}
		};

		if let Some(token) = self.peek() {
			self.last_token_position = token.position();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::input::core::Input;

	#[test]
	fn lookahead_no_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler, false);
		assert!(input.look_ahead_buffer.is_empty());
		assert_eq!(input.consumed_count(), 2);
		assert_eq!(input.look_ahead_frames.len(), 0);
		assert_eq!(input.next_token(), None);
	}

	#[test]
	fn lookahead_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler, true);
		assert!(!input.look_ahead_buffer.is_empty());
		assert_eq!(input.consumed_count(), 0);
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
	}

	#[test]
	fn peek_twice() {
		let mut input = Input::new_from_chars("12".chars());
		assert_eq!(input.peek().unwrap().token(), &'1');
		assert_eq!(input.peek().unwrap().token(), &'1');
	}

	#[test]
	fn peek_twice_while_looking_ahead_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.peek().unwrap().token(), &'1');
		assert_eq!(input.peek().unwrap().token(), &'1');
		input.stop_look_ahead(handler, true);
		assert_eq!(input.peek().unwrap().token(), &'1');
	}

	#[test]
	fn peek_twice_while_looking_ahead_not_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.peek().unwrap().token(), &'1');
		assert_eq!(input.peek().unwrap().token(), &'1');
		input.stop_look_ahead(handler, false);
		assert_eq!(input.peek().unwrap().token(), &'1');
	}

	#[test]
	fn repeat_peek_look_ahead_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		input.stop_look_ahead(handler, true);
		assert_eq!(input.peek().unwrap().token(), &'1');

		let handler = input.start_look_ahead();
		assert_eq!(input.peek().unwrap().token(), &'1');
		input.stop_look_ahead(handler, true);
		assert_eq!(input.peek().unwrap().token(), &'1');
	}

	#[test]
	fn repeat_peek_look_ahead_not_backtracking() {
		let mut input = Input::new_from_chars("12".chars());
		let handler = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		input.stop_look_ahead(handler, false);
		assert_eq!(input.peek().unwrap().token(), &'2');

		let handler = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler, true);
		assert_eq!(input.peek().unwrap().token(), &'2');
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
	}

	#[test]
	fn nested_lookahead_backtrack() {
		let mut input = Input::new_from_chars("12345".chars());
		let handler1 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		let handler2 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler2, true);
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler1, true);
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
	}

	#[test]
	fn nested_lookahead_no_backtrack() {
		let mut input = Input::new_from_chars("12345".chars());
		let handler1 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		let handler2 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler2, false);
		assert_eq!(input.next_token().unwrap().token_owned(), '3');
		input.stop_look_ahead(handler1, false);
		assert_eq!(input.next_token().unwrap().token_owned(), '4');
	}

	#[test]
	fn nested_look_ahead_backtrack_first() {
		let mut input = Input::new_from_chars("12345".chars());
		let handler1 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		let handler2 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler2, true);
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler1, false);
		assert_eq!(input.next_token().unwrap().token_owned(), '3');
	}

	#[test]
	fn nested_look_ahead_backtrack_second() {
		let mut input = Input::new_from_chars("12345".chars());
		let handler1 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
		let handler2 = input.start_look_ahead();
		assert_eq!(input.next_token().unwrap().token_owned(), '2');
		input.stop_look_ahead(handler2, false);
		assert_eq!(input.next_token().unwrap().token_owned(), '3');
		input.stop_look_ahead(handler1, true);
		assert_eq!(input.next_token().unwrap().token_owned(), '1');
	}

	#[test]
	#[should_panic]
	fn wrong_token() {
		let mut input = Input::new_from_chars("12345".chars());
		let handler1 = input.start_look_ahead();
		let _handler2 = input.start_look_ahead();
		input.stop_look_ahead(handler1, false);
	}
}
