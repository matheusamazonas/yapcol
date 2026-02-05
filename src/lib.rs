use crate::error::Error;

mod error;

struct Parser<'a, I, O>
where
	I: PartialEq + Clone
{
	f: Box<dyn 'a + Fn(&mut Vec<I>) -> Result<O, Error>>,
}

impl <'a, I, O> Parser<'a, I, O> 
where I : PartialEq  + Clone
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
		Parser { f: Box::new(f) }
	}
}

fn satisfy<'a , I, P>(predicate: P) -> Parser<'a, I, I>
where
	I: PartialEq  + Clone,
	P: 'a + Fn(&I) -> bool
{
	let f = move |input: &mut Vec<I>| match input.get(0).cloned() {
		None => Err(Error::EndOfInput),
		Some(token) => {
			match predicate(&token) {
				true => {
					input.pop(); // Consume the input on success.
					Ok(token.clone())
				},
				false => Err(Error::UnexpectedToken),
			}
		},
	};
	Parser { f: Box::new(f) }
}

fn is<I>(i: &I) -> Parser<'_, I, I>
where
	I : PartialEq  + Clone
{
	let copy = i.clone();
	let f = move |x: &I| (*x) == copy;
	satisfy(f)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn parse_number_right() {
		let parser = is(&(1));
		let mut tokens = vec![1];
		assert_eq!(parser.parse(&mut tokens), Ok(1));
		assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
	}

	#[test]
	fn parse_number_wrong() {
		let parser = is(&(1));
		let mut tokens = vec![2];
		assert_eq!(parser.parse(&mut tokens), Err(Error::UnexpectedToken));
		assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn parse_number_empty() {
		let parser = is(&(1));
		let mut tokens = vec![];
		assert_eq!(parser.parse(&mut tokens), Err(Error::EndOfInput));
	}

	#[test]
	fn parse_string_right() {
		let hello = String::from("hello");
		let parser = is(&hello);
		let mut tokens = vec![hello.clone()];
		assert_eq!(parser.parse(&mut tokens), Ok(hello.clone()));
		assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
	}

	#[test]
	fn parse_string_wrong() {
		let hello = String::from("hello");
		let hallo = String::from("hallo");
		let parser = is(&hello);
		let mut tokens = vec![hallo.clone()];
		assert_eq!(parser.parse(&mut tokens), Err(Error::UnexpectedToken));
		assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
	}
	
	#[test]
	fn parse_number_or_first() {
		let parser1 = is(&(1));
		let parser2 = is(&(2));
		let mut tokens = vec![1];
		let parser_or = parser1.or(&parser2);
		assert_eq!(parser_or.parse(&mut tokens), Ok(1));
		assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
	}

	#[test]
	fn parse_number_or_second() {
		let parser1 = is(&(1));
		let parser2 = is(&(2));
		let mut tokens = vec![2];
		let parser_or = parser1.or(&parser2);
		assert_eq!(parser_or.parse(&mut tokens), Ok(2));
		assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
	}

	#[test]
	fn parse_number_or_none() {
		let parser1 = is(&(1));
		let parser2 = is(&(2));
		let mut tokens = vec![3];
		let parser_or = parser1.or(&parser2);
		assert_eq!(parser_or.parse(&mut tokens), Err(Error::UnexpectedToken));
		assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
	}
}
