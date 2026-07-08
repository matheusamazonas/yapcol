use crate::{Error, InputToken, Parser};

/// Applies each parser in `parsers` in order, returning the result of the first one that succeeds.
/// Fails if all parsers fail.
///
/// # Outcome
///
/// This combinator succeeds if at least one element of its `parsers` argument succeeds. In this
/// case, the returned value will be the outcome of the succeeding parser.
///
/// This combinator fails if all the elements in its `parser` argument fail.
///
/// # Input consumption
///
/// This combinator consumes input if:
/// - It succeeds, and the succeeding argument parser consumes input upon success.
/// - It succeeds, but a previous argument parser failed while consuming input.
/// - It fails, and at least one of its argument parsers failed while consuming input.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure. If you would
/// like to change that behavior (for example, if the provided parsers consume upon failure),
/// wrap the argument parsers with [`crate::attempt`].
///
/// # Arguments
///
/// - `parsers`: an iterator that contains all parsers to attempt until a success.
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
	move |input| {
		let mut first_error: Option<Error> = None;
		for parser in parsers {
			match parser(input) {
				Ok(output) => return Ok(output), // Short circuit. First match wins.
				Err(e) => first_error = first_error.or(Some(e)), // Keep the first error intact.
			}
		}
		match first_error {
			Some(error) => Err(error),
			None => {
				let fallback = Error::UnexpectedToken(input.source_name(), input.position(), None);
				Err(fallback)
			}
		}
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
		let parsers: Vec<_> = vec![parser1, parser2, parser3];
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
		let mismatch = Mismatch::new('h', 'u');
		assert_eq!(
			parser_choice(&mut input),
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn fail() {
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parsers: Vec<_> = vec![parser1, parser2, parser3];
		let parser_choice = choice(&parsers);
		let mut input = Input::new_from_chars("x".chars(), None);
		let output = parser_choice(&mut input);
		let mismatch = Mismatch::new('h', 'x');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn empty() {
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parsers: Vec<_> = vec![parser1, parser2, parser3];
		let parser_choice = choice(&parsers);
		let mut input = Input::new_from_chars("".chars(), None);
		let output = parser_choice(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('h')))));
	}

	#[test]
	fn fail_consuming_input_no_attempt() {
		// Parser that consumes input upon failure:
		let parser1 = |input: &mut StringInput| {
			is('h')(input)?;
			is('a')(input)
		};
		let parser2 = is('j');
		let parser3 = is('k');
		let parsers: Vec<Box<dyn Parser<_, _>>> =
			vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
		let parser_choice = choice(&parsers);
		let mut input = Input::new_from_chars("hello".chars(), None);
		let output = parser_choice(&mut input);
		let mismatch = Mismatch::new('a', 'e');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
		assert_eq!(any()(&mut input), Ok('e')); // Input was consumed.
	}

	#[test]
	fn fail_consuming_input_attempt() {
		// Parser that consumes input upon failure:
		let parser1 = |input: &mut StringInput| {
			is('h')(input)?;
			is('a')(input)
		};
		let parser2 = is('j');
		let parser3 = is('k');
		// Wrapped in `attempt`
		let parsers: Vec<Box<dyn Parser<_, _>>> = vec![
			Box::new(parser1.attempt()),
			Box::new(parser2.attempt()),
			Box::new(parser3.attempt()),
		];
		let parser_choice = choice(&parsers);
		let mut input = Input::new_from_chars("hello".chars(), None);
		let output = parser_choice(&mut input);
		let mismatch = Mismatch::new('a', 'e');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
		assert_eq!(any()(&mut input), Ok('h')); // Input was NOT consumed.
	}
}
