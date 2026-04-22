use crate::Parser;
use crate::error::Error;
use crate::input::core::InputToken;

/// Succeeds if `parser` fails. This combinator does not consume input, even if `parser` does.
///
/// # Arguments
///
/// - `parser`: the parser which should fail for this combinator to succeed.
///
/// # Examples
///
/// ```
/// use yapcol::{is, not_followed_by};
/// use yapcol::error::Error;
/// use yapcol::input::core::Input;
/// use yapcol::input::position::Position;
///
/// let parser = is('j');
/// let mut input = Input::new_from_chars("hello".chars(), None);
/// let not_followed_parser = not_followed_by(&parser);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(output, Ok(()));
///
/// let mut input = Input::new_from_chars("jello".chars(), None);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(output, Err(Error::UnexpectedToken(None, Position::new(1,1))));
///
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
			)),
			Err(Error::EndOfInput) => Err(Error::EndOfInput),
			Err(_) => Ok(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let not_followed_parser = not_followed_by(&parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput));
	}

	#[test]
	fn followed() {
		let parser = is('h');
		let mut input = Input::new_from_chars("h".chars(), None);
		let not_followed_parser = not_followed_by(&parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
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
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
	}
}
