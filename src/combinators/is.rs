use crate::{Error, InputToken, Mismatch, Parser};

/// Creates a parser that succeeds if the next token in the input equals `token`.
///
/// If the token matches, it is consumed and returned. If the token does not match, the parser
/// fails without consuming any input.
///
/// # Arguments
///
/// - `token`: A reference to the token to match against.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, is};
///
/// let tokens: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut input = Input::new_from_chars(tokens, None);
/// let parser = is('h');
/// assert!(parser(&mut input).is_ok());
///
/// let mut wrong: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// let mut input = Input::new_from_chars(wrong, None);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('w')); // Input was not consumed.
/// ```
pub fn is<IT>(token: IT::Token) -> impl Parser<IT, IT::Token>
where
	IT: InputToken + 'static,
{
	move |input| {
		match input.peek() {
			Some(input_token) => {
				if token == *input_token.token() {
					input.next_token(); // Consume if successful.
					Ok(token.clone())
				} else {
					let position = input_token.position();
					let expected = token.clone();
					let found = (*input_token.token()).clone();
					let mismatch = Mismatch::new(expected, found);
					Err(Error::UnexpectedToken(
						input.source_name(),
						position,
						Some(mismatch),
					))
				}
			}
			None => Err(Error::EndOfInput(Some(Box::new(token.clone())))),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn success() {
		let parser = is('h');
		let mut input = Input::new_from_chars("h".chars(), None);
		assert_eq!(parser(&mut input), Ok('h'));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn fail() {
		let parser = is('j');
		let mut input = Input::new_from_chars("h".chars(), None);
		let mismatch = Mismatch::new('j', 'h');
		assert_eq!(
			parser(&mut input),
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}
}
