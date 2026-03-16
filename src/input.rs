use std::collections::VecDeque;
use std::iter::Peekable;

pub trait Token: PartialEq + Clone {}

impl<T> Token for T where T: PartialEq + Clone {}

struct LookAheadFrame {
	start_index: usize,
	length: usize,
}

impl LookAheadFrame {
	fn next_ix(&self) -> usize {
		self.start_index + self.length
	}
}

enum TokenLocation {
	Stream,
	StreamLookingAhead,
	BufferHead,
	BufferTail,
}

pub struct Input<I>
where
	I: Iterator<Item: Token>,
{
	stream: Peekable<I>,
	consumed_count: usize,
	next_location: TokenLocation,
	look_ahead_frames: Vec<LookAheadFrame>,
	look_ahead_buffer: VecDeque<I::Item>,
}

impl<I> Input<I>
where
	I: Iterator<Item: Token>,
{
	pub fn new<T>(i: impl IntoIterator<Item = T, IntoIter = I>) -> Input<I>
	where
		I: Iterator<Item = T>,
	{
		Self {
			stream: i.into_iter().peekable(),
			consumed_count: 0,
			next_location: TokenLocation::Stream,
			look_ahead_frames: Vec::new(),
			look_ahead_buffer: VecDeque::new(),
		}
	}

	pub(crate) fn next_token(&mut self) -> Option<I::Item> {
		match self.next_location {
			TokenLocation::Stream => {
				self.consumed_count += 1;
				self.stream.next()
			}
			TokenLocation::StreamLookingAhead => {
				let frame = self.look_ahead_frames.last_mut().unwrap();
				match self.stream.next() {
					None => None,
					Some(token) => {
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
				output
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
				Some(token.clone())
			}
		}
	}

	pub(crate) fn peek(&mut self) -> Option<&I::Item> {
		match self.next_location {
			TokenLocation::Stream => self.stream.peek(),
			TokenLocation::StreamLookingAhead => self.stream.peek(),
			TokenLocation::BufferHead => self.look_ahead_buffer.front(),
			TokenLocation::BufferTail => {
				let frame = self.look_ahead_frames.last_mut().unwrap();
				self.look_ahead_buffer.get(frame.next_ix())
			}
		}
	}

	pub(crate) fn consumed_count(&self) -> usize {
		self.consumed_count
	}

	pub(crate) fn start_look_ahead(&mut self) {
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

		self.look_ahead_frames.push(new_frame);
	}

	pub(crate) fn stop_look_ahead(&mut self, backtrack: bool) {
		let frame = self.look_ahead_frames.pop().unwrap();
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
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lookahead_no_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		let next = input.next_token();
		assert_eq!(next, Some(1));
		let next = input.next_token();
		assert_eq!(next, Some(2));
		input.stop_look_ahead(false);
		assert!(input.look_ahead_buffer.is_empty());
		assert_eq!(input.consumed_count(), 2);
		assert_eq!(input.look_ahead_frames.len(), 0);
		assert_eq!(input.next_token(), None);
	}

	#[test]
	fn lookahead_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		let next = input.next_token();
		assert_eq!(next, Some(1));
		let next = input.next_token();
		assert_eq!(next, Some(2));
		input.stop_look_ahead(true);
		assert!(!input.look_ahead_buffer.is_empty());
		assert_eq!(input.consumed_count(), 0);
		assert_eq!(input.next_token(), Some(1));
	}

	#[test]
	fn peek_twice() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		assert_eq!(input.peek(), Some(&1));
		assert_eq!(input.peek(), Some(&1));
	}

	#[test]
	fn peek_twice_while_looking_ahead_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.peek(), Some(&1));
		assert_eq!(input.peek(), Some(&1));
		input.stop_look_ahead(true);
		assert_eq!(input.peek(), Some(&1));
	}

	#[test]
	fn peek_twice_while_looking_ahead_not_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.peek(), Some(&1));
		assert_eq!(input.peek(), Some(&1));
		input.stop_look_ahead(false);
		assert_eq!(input.peek(), Some(&1));
	}

	#[test]
	fn repeat_peek_look_ahead_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.stop_look_ahead(true);
		assert_eq!(input.peek(), Some(&1));

		input.start_look_ahead();
		assert_eq!(input.peek(), Some(&1));
		input.stop_look_ahead(true);
		assert_eq!(input.peek(), Some(&1));
	}

	#[test]
	fn repeat_peek_look_ahead_not_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.stop_look_ahead(false);
		assert_eq!(input.peek(), Some(&2));

		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(true);
		assert_eq!(input.peek(), Some(&2));
		assert_eq!(input.next_token(), Some(2));
	}

	#[test]
	fn nested_lookahead_backtrack() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(true);
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(true);
		assert_eq!(input.next_token(), Some(1));
	}

	#[test]
	fn nested_lookahead_no_backtrack() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(false);
		assert_eq!(input.next_token(), Some(3));
		input.stop_look_ahead(false);
		assert_eq!(input.next_token(), Some(4));
	}

	#[test]
	fn nested_look_ahead_backtrack_first() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(true);
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(false);
		assert_eq!(input.next_token(), Some(3));
	}

	#[test]
	fn nested_look_ahead_backtrack_second() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(1));
		input.start_look_ahead();
		assert_eq!(input.next_token(), Some(2));
		input.stop_look_ahead(false);
		assert_eq!(input.next_token(), Some(3));
		input.stop_look_ahead(true);
		assert_eq!(input.next_token(), Some(1));
	}
}
