struct Parser<'a, I, O>
where
	I: PartialEq + Copy
{
	f: Box<dyn 'a + Fn(&mut Vec<I>) -> Result<O, String>>,
}

impl <'a, I, O> Parser<'a, I, O> 
where I : PartialEq  + Copy
{
	pub fn new<F>(f: F) -> Self
	where
		F: 'a + Fn(&mut Vec<I>) -> Result<O, String>
	{
		Parser { f: Box::new(f) }
	}
	
	pub fn parse(&self, mut input: Vec<I>) -> Result<O, String> {
		(self.f)(&mut input)
	}
}

fn satisfy<'a , I, P>(predicate: P) -> Parser<'a, I, I>
where
	I: PartialEq  + Copy,
	P: 'a + Fn(&I) -> bool
{
	let f = move |input: &mut Vec<I>| match input.pop() {
		None => Err(String::from("End of input")),
		Some(token) => match predicate(&token) {
			true => Ok(token),
			false => Err(String::from("Invalid token")),
		},
	};
	Parser { f: Box::new(f) }
}

fn is<I>(i: &I) -> Parser<'_, I, I>
where
	I : PartialEq  + Copy
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
		assert_eq!(parser.parse(vec![1]), Ok(1));
		assert_eq!(parser.parse(vec![2]), Err(String::from("Invalid token")));
	}

	#[test]
	fn parse_number_wrong() {
		let parser = is(&(1));
		assert_eq!(parser.parse(vec![2]), Err(String::from("Invalid token")));
	}

	#[test]
	fn parse_number_empty() {
		let parser = is(&(1));
		assert_eq!(parser.parse(vec![]), Err(String::from("End of input")));
	}
}
