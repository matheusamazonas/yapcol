use crate::{Error, InputToken, Parser};

/// Creates a parser that succeeds only if the input stream is empty.
///
/// If the input is empty, the parser succeeds and returns `()`. If the input still has tokens,
/// the parser fails without consuming any input.
///
/// # Examples
///
/// ```
/// use yapcol::{Error, Input, any, end_of_input};
///
/// let tokens: Vec<char> = vec![];
/// let mut input = Input::new_from_chars(tokens, None);
/// assert!(end_of_input()(&mut input).is_ok());
///
/// let tokens: Vec<char> = vec!['a', 'b'];
/// let mut input = Input::new_from_chars(tokens, None);
/// assert!(end_of_input()(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('a')); // Input was not consumed.
/// ```
pub fn end_of_input<IT>() -> impl Parser<IT, ()>
where
	IT: InputToken,
{
	|input| match input.peek() {
		None => Ok(()),
		Some(t) => {
			let position = t.position();
			Err(Error::UnexpectedToken(input.source_name(), position))
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn success() {
		let mut input = Input::new_from_chars("".chars(), None);
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn fail() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		assert!(end_of_input()(&mut input).is_err());
	}
}
