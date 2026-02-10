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
	
	pub fn then<O2>(&'a self, other: &'a Parser<'a, I, O2>) -> Parser<'a, I, (O,O2)>
	{
		let f = move |input: &mut Vec<I>| match self.parse(input) {
			Ok(o1) => match other.parse(input) {
				Ok(o2) => Ok((o1, o2)),
				Err(_) => Err(Error::UnexpectedToken)
			},
			Err(_) => Err(Error::UnexpectedToken),
		};
		Parser::new(f)
	}
	
	pub fn map<F, O2>(&'a self, f: F) -> Parser<'a, I, O2>
	where
		F: 'a + Fn(O) -> O2
	{
		let f = move |input: &mut Vec<I>| {
			match self.parse(input) {
				Ok(o) => Ok(f(o)),
				Err(_) => Err(Error::UnexpectedToken),
			}
		};
		Parser::new(f)
	}
}

fn satisfy<'a , I, O, P>(predicate: P) -> Parser<'a, I, O>
where
	P: 'a + Fn(&I) -> Result<O, Error>
{
	let f = move |input: &mut Vec<I>| match input.get(0) {
		None => Err(Error::EndOfInput),
		Some(token) => {
			match predicate(token) {
				Ok(token) => {
					input.remove(0); // Consume the input on success.
					Ok(token)
				},
				e => e,
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
	let f = move |x: &I| if (*x) == copy { Ok(x.clone()) } else { Err(Error::UnexpectedToken) };
	satisfy(f)
}
