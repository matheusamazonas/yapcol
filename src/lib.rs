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

pub fn satisfy<I, O>(p: impl Fn(&I) -> Result<O, Error>) -> impl Fn(&mut Vec<I>) -> Result<O, Error>
where
	I: Clone,
{
	move |input| match input.get(0) {
		Some(token) => {
			match p(&token) {
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

pub fn option<P, I, O>(p1: P, p2: P) -> impl Fn(&mut Vec<I>) -> Result<O, Error>
where
	P: Fn(&mut Vec<I>) -> Result<O, Error>,
{
	move |input| match p1(input) {
		Ok(output) => Ok(output),
		Err(_) => p2(input),
	}
}

pub fn maybe<P, I, O>(p1: P) -> impl Fn(&mut Vec<I>) -> Result<Option<O>, Error>
where
	P: Fn(&mut Vec<I>) -> Result<O, Error>,
{
	move |input| match p1(input) {
		Ok(output) => Ok(Some(output)),
		Err(_) => Ok(None),
	}
}