use std::iter::Peekable;

pub struct Input<I> 
where
	I: Iterator,
	I::Item: PartialEq,
{
	stream: Peekable<I>,
	consumed_count: u32
}

impl <I> Input<I>
where
	I: Iterator,
	I::Item: PartialEq,
{
	pub fn new<T>(i: impl IntoIterator<Item = T, IntoIter =I>) -> Input<I> 
	where
		I: Iterator<Item = T>,
	{
		Self { 
			stream: i.into_iter().peekable(), 
			consumed_count: 0 
		}
	}

	pub fn next_token(&mut self) -> Option<I::Item> {
		match self.stream.next() {
			Some(item) => {
				self.consumed_count += 1;
				Some(item)
			},
			None => None
		}
	}
	
	pub fn peek(&mut self) -> Option<&I::Item> {
		self.stream.peek()
	}
	
	pub fn consumed_count(&self) -> u32 {
		self.consumed_count
	}
}