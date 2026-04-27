use crate::Parser;
use crate::{Error, InputToken};

/// A simple combinator that returns the next token in the input, if any.
///
/// # Examples
///
/// ```
/// use yapcol::any;
/// use yapcol::input::Input;
///
/// // An example input iterator
/// let mut input = Input::new_from_chars("123".chars(), None);
/// let output = any()(&mut input);
/// assert_eq!(output, Ok('1'));
/// ```
pub fn any<IT>() -> impl Parser<IT, IT::Token>
where
	IT: InputToken,
{
	|input| match input.next_token() {
		Some(input_token) => Ok(input_token.token_owned()),
		None => Err(Error::EndOfInput(None)),
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn empty() {
		let mut input = Input::new_from_chars("".chars(), None);
		let output = any()(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(None)));
	}

	#[test]
	fn success() {
		let mut input = Input::new_from_chars("abc".chars(), None);
		let output = any()(&mut input);
		assert_eq!(output, Ok('a'));
		// Using it twice returns the second token.
		let output = any()(&mut input);
		assert_eq!(output, Ok('b'));
	}
}
