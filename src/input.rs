use std::ops::Index;

pub trait InputStream : Index<usize, Output = Self::Token> {
	type Token : Clone;
	
	fn len(&self) -> usize;
	fn next(&mut self) -> Option<Self::Token>;
	fn next_as_ref(&mut self) -> Option<&Self::Token>;
	fn remove_next(&mut self);
	fn append(&mut self, item: Self::Token);
	fn prepend(&mut self, item: Self::Token);
	fn prepend_many(&mut self, items: Self);
	fn copy_range(&self, from: usize, to: usize) -> Self;
	fn peek(&self) -> Self;
}

impl<I> InputStream for Vec<I>
where
	I : Clone
{
	type Token = I;
	
	fn len(&self) -> usize {
		self.len()
	}
	
	fn next(&mut self) -> Option<I> {
		if self.is_empty() {
			None
		} else {
			Some(self.remove(0))
		}
	}

	fn next_as_ref(&mut self) -> Option<&Self::Token> {
		self.first()
	}

	fn remove_next(&mut self) {
		self.remove(0);
	}

	fn append(&mut self, item: I) {
		self.push(item);
	}
	
	fn prepend(&mut self, item: I) {
		self.insert(0, item);
	}
	
	fn prepend_many(&mut self, items: Vec<I>) {
		self.splice(0..0, items);
	}

	fn copy_range(&self, from: usize, to: usize) -> Self {
		self[from..to].to_vec()
	}

	fn peek(&self) -> Self {
		match self.first() {
			Some(item) => vec![item.clone()],
			None => Vec::new()
		}
	}
}