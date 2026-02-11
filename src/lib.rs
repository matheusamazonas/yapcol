use crate::error::Error;

#[cfg(test)]
mod tests;
mod error;

pub fn is<I>(i: &I) -> impl Fn(&mut Vec<I>) -> Result<I, Error>
where
	I: PartialEq + Clone,
{
	let f = move |x: &I| match *x == *i {
		true => Ok(x.clone()),
		false => Err(Error::UnexpectedToken)
	};
	satisfy(f)
}

pub fn satisfy<P,I, O>(parser: P) -> impl Fn(&mut Vec<I>) -> Result<O, Error>
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
				},
				Err(e) => Err(e),
			}
		}
		None => Err(Error::EndOfInput),
	}
}

pub fn end_of_input<I>() -> impl Fn(&mut Vec<I>) -> Result<(), Error> {
	|input| match input.len() {
		0 => Ok(()),
		_ => Err(Error::UnexpectedToken)
	}
}

pub fn option<P, I, O>(parser1: &P, parser2: &P) -> impl Fn(&mut Vec<I>) -> Result<O, Error>
where
	P: Fn(&mut Vec<I>) -> Result<O, Error>,
{
	move |input| match parser1(input) {
		Ok(token) => Ok(token),
		Err(_) => parser2(input),
	}
}

pub fn maybe<P, I, O>(parser: &P) -> impl Fn(&mut Vec<I>) -> Result<Option<O>, Error>
where
	P: Fn(&mut Vec<I>) -> Result<O, Error>,
{
	move |input| match parser(input) {
		Ok(token) => Ok(Some(token)),
		Err(_) => Ok(None),
	}
}