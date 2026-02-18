use crate::error::Error;
use crate::input::InputStream;

mod error;
#[cfg(test)]
mod tests;
mod input;

pub trait Parser<I, O>: Fn(&mut I) -> Result<O, Error> 
where
I: InputStream,
{}

impl<I, O, T> Parser<I, O> for T 
where 
	I: InputStream,
	T: Fn(&mut I) -> Result<O, Error> { }

pub fn is<I, T>(i: &T) -> impl Parser<I, T>
where
	I: InputStream<Token = T>,
	T: PartialEq + Clone,
{
	let f = |x: &T| match *x == *i {
		true => Ok(x.clone()),
		false => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

pub fn satisfy<F, I, O, T>(f: F) -> impl Parser<I, O>
where
	I: InputStream<Token = T>,
	F: Fn(&T) -> Result<O, Error>,
{
	move |input| match input.next() {
		Some(token) => {
			match f(token) {
				Ok(result) => {
					input.remove_next(); // Consume if successful.
					Ok(result)
				}
				Err(e) => Err(e),
			}
		}
		None => Err(Error::EndOfInput),
	}
}

pub fn end_of_input<I>() -> impl Parser<I, ()> 
where
	I: InputStream,
{
	|input| match input.len() {
		0 => Ok(()),
		_ => Err(Error::UnexpectedToken),
	}
}

pub fn option<P1, P2, I, O>(parser1: &P1, parser2: &P2) -> impl Parser<I, O>
where
	I: InputStream,
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
	I: InputStream,
	P: Parser<I, O>,
{
	|input| match parser(input) {
		Ok(token) => Ok(Some(token)),
		Err(_) => Ok(None),
	}
}

fn many<P, I, O>(parser: &P) -> impl Fn(&mut I, Vec<O>) -> Result<Vec<O>, Error>
where
	I: InputStream,
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
	I: InputStream,
	P: Parser<I, O>,
{
	|input| {
		let output: Vec<O> = Vec::new();
		many(parser)(input, output)
	}
}

pub fn many1<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	I: InputStream,
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
	I: InputStream,
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

pub fn count<P, I, O>(parser: &P, count: usize) -> impl Parser<I, Vec<O>>
where
	I: InputStream,
	P: Parser<I, O>,
{
	move |input| {
		let tokens = input.copy_range(0, count); // Keep a copy of the tokens if we need to rewind.
		let mut output = Vec::with_capacity(count);
		for i in 0..count {
			match parser(input) {
				Ok(token) => output.push(token),
				Err(_) => {
					// Abort, rewinding. Add past success tokens back to input.
					input.prepend_many(tokens.copy_range(0, i));
					return Err(Error::UnexpectedToken)
				},
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