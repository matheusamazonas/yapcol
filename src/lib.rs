use crate::error::Error;

#[cfg(test)]
mod tests;
mod error;

struct Parser<'a, I, O>
{
	f: Box<dyn 'a + Fn(&mut Vec<I>) -> Result<O, Error>>,
}

impl <'a, I, O> Parser<'a, I, O>
{
	pub fn new<F>(f: F) -> Self
	where
		F: 'a + Fn(&mut Vec<I>) -> Result<O, Error>
	{
		Parser { f: Box::new(f) }
	}
	
	pub fn parse(&self, input: &mut Vec<I>) -> Result<O, Error> {
		match (self.f)(input) {
			Ok(o) => Ok(o),
			Err(e) => Err(e)
		}
	}
	
	pub fn or(&'a self, other: &'a Parser<'a, I, O>) -> Parser<'a, I, O> {
		let f = move |input: &mut Vec<I>| match self.parse(input) {
			Ok(t) => Ok(t),
			Err(_) => other.parse(input),
		};
		Parser::new(f)
	}
	
	pub fn maybe(&'a self) -> Parser<'a, I, Option<O>> {
		let f = move |input: &mut Vec<I>| match self.parse(input) {
			Ok(o) => Ok(Some(o)),
			Err(_) => Ok(None),
		};
		Parser::new(f)
	}
}

fn satisfy<'a , I, P>(predicate: P) -> Parser<'a, I, I>
where
	P: 'a + Fn(&I) -> bool
{
	let f = move |input: &mut Vec<I>| match input.get(0) {
		None => Err(Error::EndOfInput),
		Some(token) => {
			match predicate(&token) {
				true => {
					input.remove(0); // Consume the input on success.
					Ok(token.clone())
				},
				false => Err(Error::UnexpectedToken),
			}
		},
	};
	Parser::new(f)
}

fn is<I>(i: &I) -> Parser<'_, I, I>
where
	I : PartialEq  + Clone
{
	let copy = i.clone();
	let f = move |x: &I| (*x) == copy;
	satisfy(f)
}
