use std::collections::VecDeque;
use std::iter::Peekable;

pub trait Token: PartialEq + Clone {}

impl<T> Token for T where T: PartialEq + Clone {}

pub struct Input<I>
where
	I: Iterator<Item: Token>,
{
	stream: Peekable<I>,
	consumed_count: u32,
	is_peeking: bool,
	next_on_buffer: bool,
	peek_buffer: VecDeque<I::Item>,
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
			is_peeking: false,
			next_on_buffer: false,
			peek_buffer: VecDeque::new(),
		}
	}

	pub fn next_token(&mut self) -> Option<I::Item> {
		if self.is_peeking {
			match self.stream.next() {
				None => None,
				Some(token) => {
					let cloned = token.clone();
					self.peek_buffer.push_back(token);
					Some(cloned)
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
		if self.next_on_buffer {
			self.peek_buffer.front()
		} else {
			self.stream.peek()
		}
	}

	pub fn consumed_count(&self) -> u32 {
		self.consumed_count
	}

	pub fn start_peeking(&mut self) {
		self.is_peeking = true;
	}

	pub fn stop_peeking(&mut self, backtrack: bool) {
		self.is_peeking = false;
		if backtrack {
			self.next_on_buffer = !self.peek_buffer.is_empty();
		} else {
			self.consumed_count += self.peek_buffer.len() as u32;
			self.peek_buffer.clear();
			self.next_on_buffer = false;
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
		assert!(!input.is_peeking);
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
		assert!(!input.is_peeking);
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
}
