use crate::{Error, InputToken, Parser};

/// Applies each parser in `parsers` in order, returning the result of the first one that succeeds.
/// Fails if all parsers fail.
///
/// # Arguments
///
/// - `parsers`: An iterator that contains all parsers to attempt until a success.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, choice, is};
///
/// // Returns the result of the first matching parser
/// let p1 = is('1');
/// let p2 = is('2');
/// let parsers = vec![p1, p2];
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert_eq!(choice(&parsers)(&mut input), Ok('2'));
///
/// // Fails when no parser matches
/// let mut input = Input::new_from_chars("34".chars(), None);
/// assert!(choice(&parsers)(&mut input).is_err());
///
/// // Fails on empty input
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(choice(&parsers)(&mut input).is_err());
/// ```
pub fn choice<'a, P, IT, O, PI>(parsers: &'a PI) -> impl Parser<IT, O>
where
	P: Parser<IT, O> + 'a,
	IT: InputToken,
	&'a PI: IntoIterator<Item = &'a P>,
{
	|input| {
		parsers
			.into_iter()
			.find_map(|p| p(input).ok())
			.ok_or(Error::UnexpectedToken(
				input.source_name(),
				input.position(),
			))
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn success() {
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parsers: Vec<Box<dyn Parser<_, _>>> =
			vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
		let parser_choice = choice(&parsers);
		// 1, success.
		let mut input = Input::new_from_chars("h".chars(), None);
		let output = parser_choice(&mut input).unwrap();
		assert_eq!(output, 'h');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		// 2, success.
		let mut input = Input::new_from_chars("e".chars(), None);
		let output = parser_choice(&mut input).unwrap();
		assert_eq!(output, 'e');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		// 3, success.
		let mut input = Input::new_from_chars("l".chars(), None);
		let output = parser_choice(&mut input).unwrap();
		assert_eq!(output, 'l');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		// 4, fail.
		let mut input = Input::new_from_chars("u".chars(), None);
		assert_eq!(
			parser_choice(&mut input),
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn fail() {
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parsers: Vec<Box<dyn Parser<_, _>>> =
			vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
		let parser_choice = choice(&parsers);
		// 1, success.
		let mut input = Input::new_from_chars("x".chars(), None);
		let output = parser_choice(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}
}
