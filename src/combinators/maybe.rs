use crate::{InputToken, Parser};

/// Creates a parser that makes another parser optional.
///
/// If the input parser succeeds, its result is wrapped in `Some` and returned. If the input
/// parser fails **without consuming any input**, the returned parser succeeds with `Ok(None)`.
/// If the input parser fails **after consuming input**, the error is propagated as `Err`.
///
/// # Arguments
///
/// - `parser`: The parser to make optional.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, is, maybe};
///
/// let mut input = Input::new_from_chars("hello".chars(), None);
/// let ph = is('h');
/// let parser = maybe(&ph);
/// assert_eq!(parser(&mut input).unwrap(), Some('h'));
///
/// let mut input = Input::new_from_chars("world".chars(), None);
/// assert_eq!(parser(&mut input).unwrap(), None);
/// assert_eq!(any()(&mut input), Ok('w')); // Input was not consumed.
/// ```
pub fn maybe<P, IT, O>(parser: &P) -> impl Parser<IT, Option<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let initial_length = input.consumed_count();
		match parser(input) {
			Ok(token) => Ok(Some(token)),
			Err(_) if input.consumed_count() == initial_length => Ok(None),
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::input::StringInput;
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let parser_maybe = maybe(&parser);
		assert_eq!(parser_maybe(&mut input), Ok(None));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').maybe();
		let mut input = Input::new_from_chars("".chars(), None);
		assert_eq!(parser(&mut input), Ok(None));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn success() {
		let parser = is('h');
		let mut input = Input::new_from_chars("h".chars(), None);
		let parser_maybe = maybe(&parser);
		assert_eq!(parser_maybe(&mut input), Ok(Some('h')));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn success_shortcut() {
		let parser = is('h').maybe();
		let mut input = Input::new_from_chars("h".chars(), None);
		assert_eq!(parser(&mut input), Ok(Some('h')));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn fail_non_consuming() {
		let parser = is('h');
		let mut input = Input::new_from_chars("j".chars(), None);
		let parser_maybe = maybe(&parser);
		assert_eq!(parser_maybe(&mut input), Ok(None));
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn fail_consuming() {
		let parser = |input: &mut StringInput| match any()(input) {
			Ok(token) => {
				if token == 'h' {
					Ok(1)
				} else {
					Err(Error::UnexpectedToken(None, Position::new(1, 1)))
				}
			}
			Err(e) => Err(e),
		};
		let mut input = Input::new_from_chars("j".chars(), None);
		let parser_maybe = maybe(&parser);
		assert_eq!(
			parser_maybe(&mut input),
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}
}
