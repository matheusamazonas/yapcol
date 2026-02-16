use crate::error::Error;

mod error;
#[cfg(test)]
mod tests;

pub trait Parser<I, O>: Fn(&mut Vec<I>) -> Result<O, Error> {}

impl<I, O, T> Parser<I, O> for T where T: Fn(&mut Vec<I>) -> Result<O, Error> {}

pub fn is<I>(i: &I) -> impl Parser<I, I>
where
	I: PartialEq + Clone,
{
	let f = |x: &I| match *x == *i {
		true => Ok(x.clone()),
		false => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

pub fn satisfy<F, I, O>(f: F) -> impl Parser<I, O>
where
	F: Fn(&I) -> Result<O, Error>,
{
	move |input| match input.first() {
		Some(token) => {
			match f(token) {
				Ok(result) => {
					input.remove(0); // Consume if successful.
					Ok(result)
				}
				Err(e) => Err(e),
			}
		}
		None => Err(Error::EndOfInput),
	}
}

pub fn end_of_input<I>() -> impl Parser<I, ()> {
	|input| match input.len() {
		0 => Ok(()),
		_ => Err(Error::UnexpectedToken),
	}
}

pub fn option<P1, P2, I, O>(parser1: &P1, parser2: &P2) -> impl Parser<I, O>
where
	P1: Parser<I, O>,
	P2: Parser<I, O>,
{
	|input| match parser1(input) {
		Ok(token) => Ok(token),
		Err(_) => parser2(input),
	}
}

pub fn maybe<P, I, O>(parser: &P) -> impl Parser<I, Option<O>>
where
	P: Fn(&mut Vec<I>) -> Result<O, Error>,
{
	|input| match parser(input) {
		Ok(token) => Ok(Some(token)),
		Err(_) => Ok(None),
	}
}

fn many<P, I, O>(parser: &P) -> impl Fn(&mut Vec<I>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<I, O>,
{
	|input, mut output| match parser(input) {
		Ok(token) => {
			output.push(token);
			many(parser)(input, output)
		}
		Err(_) => Ok(output),
	}
}

pub fn many0<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
{
	|input| {
		let output: Vec<O> = Vec::new();
		many(parser)(input, output)
	}
}

pub fn many1<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
{
	|input| {
		let mut output: Vec<O> = Vec::new();
		match parser(input) {
			Ok(token) => {
				output.push(token);
				many(parser)(input, output)
			}
			Err(e) => Err(e),
		}
	}
}

pub fn choice<'a, P, I, O, PI>(parsers: &'a PI) -> impl Parser<I, O>
where
	P: Parser<I, O> + 'a,
	&'a PI: IntoIterator<Item = &'a P>,
{
	|input| {
		parsers
			.into_iter()
			.find_map(|p| p(input).ok())
			.ok_or(Error::UnexpectedToken)
	}
}

pub fn count<P, I, O>(parser: &P, count: u32) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
{
	move |input| {
		let mut output: Vec<O> = Vec::new();
		for _ in 0..count {
			match parser(input) {
				Ok(token) => output.push(token),
				Err(_) => break,
			}
		}
		Ok(output)
	}
}

// TO DO list:
// - between
// - separated by (0, 1)
// - chain left (0, 1)
// - chain right (0, 1)
// - look ahead
// - any