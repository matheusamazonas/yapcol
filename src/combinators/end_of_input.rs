use crate::{Error, InputToken, Mismatch, Parser};

/// Creates a parser that succeeds only if the input stream is empty.
///
/// # Outcome
///
/// This parser succeeds and returns `()` if the input is empty. Otherwise, it fails with an
/// [`Error::UnexpectedToken`].
///
/// # Input consumption
///
/// This parser never consumes input tokens.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
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
		Some(token) => {
			let position = token.position();
			let mismatch = Mismatch::new("end of input", token.token().clone().to_string());
			Err(Error::UnexpectedToken(
				input.source_name(),
				position,
				Some(mismatch),
			))
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
