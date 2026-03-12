use std::collections::VecDeque;
use std::iter::Peekable;

pub trait Token: PartialEq + Clone {}

impl<T> Token for T where T: PartialEq + Clone {}

pub struct Input<I>
where
	I: Iterator<Item: Token>,
{
	stream: Peekable<I>,
	consumed_count: usize,
	next_on_buffer: bool,
	frames: Vec<LookAheadFrame>,
	peek_buffer: VecDeque<I::Item>,
}

struct LookAheadFrame {
	start_index: usize,
	length: usize,
}

impl LookAheadFrame {
	fn next_ix(&self) -> usize {
		self.start_index + self.length
	}
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
			next_on_buffer: false,
			frames: Vec::new(),
			peek_buffer: VecDeque::new(),
		}
	}

	pub fn next_token(&mut self) -> Option<I::Item> {
		if !self.frames.is_empty() {
			let frame = self.frames.last_mut().unwrap();
			if !self.peek_buffer.is_empty() && frame.next_ix() < self.peek_buffer.len() {
				let output = self.peek_buffer.get(frame.next_ix()).unwrap();
				frame.length += 1;
				Some(output.clone())
			} else {
				match self.stream.next() {
					None => None,
					Some(token) => {
						let cloned = token.clone();
						self.peek_buffer.push_back(token);
						frame.length += 1;
						Some(cloned)
					}
				}
			}
		} else if self.next_on_buffer {
			self.consumed_count += 1;
			let output = self.peek_buffer.pop_front();
			self.next_on_buffer = !self.peek_buffer.is_empty();
			output
		} else {
			self.consumed_count += 1;
			self.stream.next()
		}
	}

	pub fn next_token_ref(&mut self) -> Option<&I::Item> {
		if !self.frames.is_empty() {
			let frame = self.frames.last_mut().unwrap();
			if !self.peek_buffer.is_empty() && frame.next_ix() < self.peek_buffer.len() {
				self.peek_buffer.get(frame.next_ix())
			} else {
				self.stream.peek()
			}
		} else if self.next_on_buffer {
			self.peek_buffer.front()
		} else {
			self.stream.peek()
		}
	}

	pub fn consumed_count(&self) -> usize {
		self.consumed_count
	}

	pub fn start_peeking(&mut self) {
		let new_frame = match self.frames.last() {
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
		self.frames.push(new_frame)
	}

	pub fn stop_peeking(&mut self, backtrack: bool) {
		let frame = self.frames.pop().unwrap();
		if backtrack {
			self.next_on_buffer = !self.peek_buffer.is_empty();
		} else {
			self.consumed_count += frame.length;
			let buffer_length = self.peek_buffer.len();
			self.peek_buffer.truncate(buffer_length - frame.length);
			self.next_on_buffer = !self.peek_buffer.is_empty();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn peek_no_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		let next = input.next_token();
		assert_eq!(next, Some(1));
		let next = input.next_token();
		assert_eq!(next, Some(2));
		input.stop_peeking(false);
		assert!(input.peek_buffer.is_empty());
		assert_eq!(input.consumed_count(), 2);
		assert_eq!(input.frames.len(), 0);
		assert_eq!(input.next_token(), None);
	}

	#[test]
	fn peek_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		let next = input.next_token();
		assert_eq!(next, Some(1));
		let next = input.next_token();
		assert_eq!(next, Some(2));
		input.stop_peeking(true);
		assert!(!input.peek_buffer.is_empty());
		assert_eq!(input.consumed_count(), 0);
		assert_eq!(input.next_token(), Some(1));
	}

	#[test]
	fn next_ref_twice() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		assert_eq!(input.next_token_ref(), Some(&1));
		assert_eq!(input.next_token_ref(), Some(&1));
	}

	#[test]
	fn peek_next_ref_twice_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token_ref(), Some(&1));
		assert_eq!(input.next_token_ref(), Some(&1));
		input.stop_peeking(true);
		assert_eq!(input.next_token_ref(), Some(&1));
	}

	#[test]
	fn peek_next_ref_twice_not_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token_ref(), Some(&1));
		assert_eq!(input.next_token_ref(), Some(&1));
		input.stop_peeking(false);
		assert_eq!(input.next_token_ref(), Some(&1));
	}

	#[test]
	fn peek_twice_next_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.stop_peeking(true);
		assert_eq!(input.next_token_ref(), Some(&1));

		input.start_peeking();
		assert_eq!(input.next_token_ref(), Some(&1));
		input.stop_peeking(true);
		assert_eq!(input.next_token_ref(), Some(&1));
	}

	#[test]
	fn peek_twice_next_ref_not_backtracking() {
		let tokens = vec![1, 2];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.stop_peeking(false);
		assert_eq!(input.next_token_ref(), Some(&2));
		input.start_peeking();
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(true);
		assert_eq!(input.next_token_ref(), Some(&2));
		assert_eq!(input.next_token(), Some(2));
	}

	#[test]
	fn nested_peek_backtrack() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.start_peeking();
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(true);
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(true);
		assert_eq!(input.next_token(), Some(1));
	}

	#[test]
	fn nested_peek_no_backtrack() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.start_peeking();
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(false);
		assert_eq!(input.next_token(), Some(3));
		input.stop_peeking(false);
		assert_eq!(input.next_token(), Some(4));
	}

	#[test]
	fn nested_peek_backtrack_first() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.start_peeking();
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(true);
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(false);
		assert_eq!(input.next_token(), Some(3));
	}

	#[test]
	fn nested_peek_backtrack_second() {
		let tokens = vec![1, 2, 3, 4, 5];
		let mut input = Input::new(tokens);
		input.start_peeking();
		assert_eq!(input.next_token(), Some(1));
		input.start_peeking();
		assert_eq!(input.next_token(), Some(2));
		input.stop_peeking(false);
		assert_eq!(input.next_token(), Some(3));
		input.stop_peeking(true);
		assert_eq!(input.next_token(), Some(1));
	}
}
