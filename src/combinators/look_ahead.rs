use crate::{InputToken, Parser};

/// Creates a parser that does not consume input in case the given parser succeeds.
///
/// If the given parser succeeds, the matched value is returned, but the input is left unchanged.
/// If the given parser fails while consuming input, this parser also fails while consuming input.
///
/// # Arguments
///
/// - `parser`: The parser to look ahead.
///
/// # Examples
///
/// ```
/// use yapcol::{look_ahead, is, end_of_input, any};
/// use yapcol::{Error, Input};
/// use yapcol::input::Position;
///
/// // Succeeds without consuming input.
/// let mut input = Input::new_from_chars("123".chars(), None);
/// let parser1 = is('1');
/// assert_eq!(look_ahead(&parser1)(&mut input), Ok('1'));
/// assert_eq!(any()(&mut input), Ok('1')); // Input was not consumed.
/// assert_eq!(any()(&mut input), Ok('2')); // Input was not consumed.
/// assert_eq!(any()(&mut input), Ok('3')); // Input was not consumed.
///
/// // Fails without consuming input.
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert!(look_ahead(&parser1)(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('2')); // Input was not consumed.
///
/// // Fails consuming input if the parser consumes.
/// let mut input = Input::new_from_chars("13".chars(), None);
/// let consuming_parser = |input: &mut Input<_>| {
///     let o1 = parser1(input)?;
///     let o2 = parser1(input)?;
///     Ok((o1, o2))
/// };
/// let output = look_ahead(&consuming_parser)(&mut input);
/// assert_eq!(output, Err(Error::UnexpectedToken(None, Position::new(1,2))));
/// assert_eq!(any()(&mut input), Ok('3')); // Input was consumed.
///
/// // Fails on empty input.
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(look_ahead(&parser1)(&mut input).is_err());
/// ```
pub fn look_ahead<P, IT, O>(parser: &P) -> impl Parser<IT, O>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, output.is_ok());
		output
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
		let parse_look_ahead = look_ahead(&parser)(&mut input);
		assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
	}

	#[test]
	fn success_does_not_consume() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let output = look_ahead(&parser)(&mut input);
		assert_eq!(output, Ok('h'));
		// After look_ahead, input should still start with hello
		assert_eq!(is('h')(&mut input), Ok('h'));
		assert_eq!(is('e')(&mut input), Ok('e'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert_eq!(is('o')(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn non_consuming_fail_does_not_consume() {
		let parser = is('h');
		let mut input = Input::new_from_chars("j".chars(), None);
		let output = look_ahead(&parser)(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		// Input should still be intact.
		assert_eq!(any()(&mut input), Ok('j'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn consuming_fail_consumes() {
		let mut input = Input::new_from_chars("he".chars(), None);
		let parser = |input: &mut Input<_>| {
			let output1 = is('h')(input)?; // Success, therefore it consumed.
			let output2 = is('a')(input)?; // Failed, so the whole parser fails while consuming.
			Ok((output1, output2))
		};
		let output = look_ahead(&parser)(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 2)))
		);
		// Input was consumed.
		assert_eq!(any()(&mut input), Ok('e'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn parse_does_not_consume_on_failure() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jello".chars(), None);
		let result = look_ahead(&parser)(&mut input);
		assert_eq!(
			result,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		// Input should still be intact
		assert_eq!(is('j')(&mut input), Ok('j'));
		assert_eq!(is('e')(&mut input), Ok('e'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert_eq!(is('o')(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn parse_look_ahead_twice() {
		let parser = is('h');
		let mut input = Input::new_from_chars("h".chars(), None);
		let first = look_ahead(&parser)(&mut input);
		let second = look_ahead(&parser)(&mut input);
		assert_eq!(first, Ok('h'));
		assert_eq!(second, Ok('h'));
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}
}
