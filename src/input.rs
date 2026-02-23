use std::ops::Index;

pub trait InputStream : Index<usize, Output = Self::Token> {
	type Token : Clone;
	
	fn len(&self) -> usize;
	fn next(&mut self) -> Option<Self::Token>;
	fn next_as_ref(&mut self) -> Option<&Self::Token>;
	fn remove_next(&mut self);
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

	fn peek(&self) -> Self {
		match self.first() {
			Some(item) => vec![item.clone()],
			None => Vec::new()
		}
	}
}