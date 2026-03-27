//! This module provides the [`Input`] type, which wraps any iterator and exposes it as a token
//! stream that parsers can consume. It is the primary interface through which parsers read tokens,
//! and it is the only type in this module that users need to interact with directly.
//!
//! # Tokens
//!
//! Any type that implements [`PartialEq`] and [`Clone`] can be used as a token. This is
//! automatically satisfied via the blanket implementation of the [`Token`] trait, so no manual
//! implementation is required.
//!
//! # Creating an input stream
//!
//! An [`Input`] is constructed from any type that implements [`IntoIterator`], where the iterator
//! elements must implement [`Token`]:
//!
//! ```
//! use yapcol::input::Input;
//!
//! let tokens = vec!['h', 'e', 'l', 'l', 'o'];
//! let mut input = Input::new(tokens);
//! ```
//!
//! # Lookahead
//!
//! [`Input`] supports arbitrary lookahead: tokens can be fetched and inspected without being
//! permanently consumed. A lookahead operation is started with
//! [`Input::start_look_ahead`](Input::start_look_ahead) and stopped with
//! [`Input::stop_look_ahead`](Input::stop_look_ahead). When stopping, the caller decides whether
//! to backtrack (keeping the fetched tokens available for re-reading) or to discard them.
//!
//! Lookahead operations can be nested. Each nested operation is self-contained, and they must be
//! stopped in the reverse order of their creation (last started, first stopped).
//!
//! In practice, lookahead is used internally by parser combinators such as [`crate::attempt`] and
//! [`crate::look_ahead`] defined in the crate root, so most users will not need to call these
//! methods directly.

pub mod string;
pub mod token;

#[cfg(test)]
mod tests;

use std::collections::VecDeque;
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

pub trait InputToken: Clone {
	type Token: PartialEq + Clone;
	fn token(&self) -> &Self::Token;
	fn token_owned(self) -> Self::Token;
	fn position(&self) -> Position;
}

/// A frame used to keep track of lookahead operations.
struct LookAheadFrame {
	/// Where in the lookahead buffer this frame started.
	start_index: usize,
	/// How many tokens this frame takes in the lookahead buffer.
	length: usize,
}

impl LookAheadFrame {
	/// The (lookahead buffer) index of the next token in this frame.
	fn next_ix(&self) -> usize {
		self.start_index + self.length
	}
}

/// A handler used to enforce the following on lookahead operations:
/// - That calls to [`Input::stop_look_ahead`] are only possible after a call to
///   [`Input::start_look_ahead`].
/// - That the right order of start/stop lookahead operations is performed.
#[must_use]
pub(crate) struct LookAheadHandler {
	id: usize,
}

/// Possible locations where the next input token might be.
enum TokenLocation {
	Stream,
	StreamLookingAhead,
	BufferHead,
	BufferTail,
}

trait InputSource {
	type Token: InputToken + Sized;
	fn source_name(&self) -> String;
	fn next_token(&mut self) -> Option<Self::Token>;
	fn peek(&mut self) -> Option<&Self::Token>;
}

/// An input stream that can be used to fetch input tokens. It's the most important entity in this
/// module, concentrating all input operations.
pub struct Input<'a, IT>
where
	IT: InputToken,
{
	source: Box<dyn InputSource<Token =IT> + 'a>,
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
		LookAheadHandler { id: token_id }
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
		if handler.id != self.look_ahead_frames.len() {
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
