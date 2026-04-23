use crate::{Error, InputToken, Parser};

/// Creates a parser that succeeds if the given predicate returns `Some` for the next token.
///
/// If the predicate succeeds, the token is consumed and the result is returned. If the predicate
/// fails, the parser fails without consuming any input.
///
/// # Arguments
///
/// - `f`: A predicate that takes a reference to a token and returns `Some` on success or
///   `None` on failure.
///
/// # Examples
///
/// ```
/// use yapcol::{satisfy, any, Error, Input};
///
/// let tokens: Vec<char> = vec!['3', 'a', 'b'];
/// let mut input = Input::new_from_chars(tokens, None);
/// let parser = satisfy(|c: &char| {
///     if c.is_ascii_digit() { Some(*c) } else { None }
/// });
/// assert_eq!(parser(&mut input).unwrap(), '3');
/// assert_eq!(any()(&mut input), Ok('a')); // Token was consumed.
///
/// let tokens: Vec<char> = vec!['a', 'b', 'c'];
/// let mut input = Input::new_from_chars(tokens, None);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('a')); // Input was not consumed.
/// ```
pub fn satisfy<F, IT, O>(f: F) -> impl Parser<IT, O>
where
	F: Fn(&IT::Token) -> Option<O>,
	IT: InputToken,
{
	move |input| {
		match input.peek() {
			Some(input_token) => {
				let token = input_token.token();
				match f(token) {
					Some(result) => {
						input.next_token(); // Consume if successful.
						Ok(result)
					}
					None => {
						let position = input_token.position();
						Err(Error::UnexpectedToken(input.source_name(), position))
					}
				}
			}
			None => Err(Error::EndOfInput),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn digits() {
		let parser = satisfy(|token: &char| {
			if token.is_ascii_digit() {
				Some(*token)
			} else {
				None
			}
		});
		// Digits.
		let mut input = Input::new_from_chars("1".chars(), None);
		assert_eq!(parser(&mut input), Ok('1'));
		assert!(end_of_input()(&mut input).is_ok());
		// Words fails and does not consume.
		let mut input = Input::new_from_chars("hello".chars(), None);
		assert_eq!(
			parser(&mut input),
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		assert_eq!(any()(&mut input), Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
	}
}
