use crate::{Error, InputToken, Parser};

/// Succeeds if `parser` fails. This combinator does not consume input, even if `parser` does.
///
/// # Arguments
///
/// - `parser`: the parser which should fail for this combinator to succeed.
///
/// # Examples
///
/// ```
/// use yapcol::input::Position;
/// use yapcol::{Error, Input, is, not_followed_by};
///
/// let parser = is('j');
/// let mut input = Input::new_from_chars("hello".chars(), None);
/// let not_followed_parser = not_followed_by(&parser);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(output, Ok(()));
///
/// let mut input = Input::new_from_chars("jello".chars(), None);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(
/// 	output,
/// 	Err(Error::UnexpectedToken(None, Position::new(1, 1), None))
/// );
/// ```
pub fn not_followed_by<P, IT, O>(parser: &P) -> impl Parser<IT, ()>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, true);
		match output {
			Ok(_) => Err(Error::UnexpectedToken(
				input.source_name(),
				input.position(),
				None,
			)),
			Err(Error::EndOfInput(oe)) => Err(Error::EndOfInput(oe)),
			Err(_) => Ok(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let not_followed_parser = not_followed_by(&parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('h')))));
	}

	#[test]
	fn followed() {
		let parser = is('h');
		let mut input = Input::new_from_chars("h".chars(), None);
		let not_followed_parser = not_followed_by(&parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1), None))
		);
	}

	#[test]
	fn not_followed() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jello".chars(), None);
		let not_followed_parser = not_followed_by(&parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Ok(()));
	}

	#[test]
	fn look_ahead_followed() {
		// Inspiration: https://github.com/haskell/parsec/issues/8
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let lookahead_parser = look_ahead(&parser);
		// Just ensure that it succeeds to prove a point.
		let output = lookahead_parser(&mut input);
		assert_eq!(output, Ok('h'));
		// Actually test.
		let mut input = Input::new_from_chars("hello".chars(), None);
		let not_followed_parser = not_followed_by(&lookahead_parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1), None))
		);
	}
}
