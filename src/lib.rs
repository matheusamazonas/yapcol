use crate::error::Error;

mod error;
#[cfg(test)]
mod tests;

pub trait Parser<I, O>: Fn(&mut Vec<I>) -> Result<O, Error> {}

impl<I, O, T> Parser<I, O> for T where T: Fn(&mut Vec<I>) -> Result<O, Error> {}

pub fn is<I>(i: &I) -> impl Fn(&mut Vec<I>) -> Result<I, Error>
where
	I: PartialEq + Clone,
{
	let f = |x: &I| match *x == *i {
		true => Ok(x.clone()),
		false => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

pub fn satisfy<P, I, O>(parser: P) -> impl Parser<I, O>
where
	P: Fn(&I) -> Result<O, Error>,
	I: Clone,
{
	move |input| match input.get(0) {
		Some(token) => {
			match parser(&token) {
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

pub fn option<P, I, O>(parser1: &P, parser2: &P) -> impl Parser<I, O>
where
	P: Parser<I, O>,
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
