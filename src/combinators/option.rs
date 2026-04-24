use crate::{InputToken, Parser};

/// Creates a parser based on two input parsers. It tries the first parser and falls back to the
/// second if the first fails without consuming input.
///
/// If `parser1` succeeds, its result is returned. If `parser1` fails without consuming any input,
/// `parser2` is applied and its result is returned. If `parser1` fails while consuming input, the
/// error is propagated and no attempt to apply `parser2` is made.
///
///  If you would like to attempt to apply `parser2` even if `parser1` failed while consuming input,
/// check the `attempt` parser combinator.
///
/// # Arguments
///
/// - `parser1`: The first parser to try.
/// - `parser2`: The fallback parser, applied only if `parser1` fails without consuming input.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, end_of_input, is, option};
///
/// // parser1 succeeds: returns its result.
/// let mut input = Input::new_from_chars("ab".chars(), None);
/// let output = option(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'a');
///
/// // parser1 fails without consuming input: parser2 is tried.
/// let mut input = Input::new_from_chars("b".chars(), None);
/// let output = option(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'b');
///
/// // Both parsers fail: an error is returned and input is not consumed.
/// let mut input = Input::new_from_chars("c".chars(), None);
/// assert!(option(&is('a'), &is('b'))(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('c'));
/// ```
pub fn option<P1, P2, IT, O>(parser1: &P1, parser2: &P2) -> impl Parser<IT, O>
where
	P1: Parser<IT, O>,
	P2: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let initial_length = input.consumed_count();
		match parser1(input) {
			Ok(token) => Ok(token),
			Err(_) if input.consumed_count() == initial_length => parser2(input),
			Err(e) => Err(e),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn success_first() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("h".chars(), None);
		let parse_option = option(&parser1, &parser2);
		let output = parse_option(&mut input).unwrap();
		assert_eq!(output, 'h');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn success_second() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("j".chars(), None);
		let parse_option = option(&parser1, &parser2);
		let output = parse_option(&mut input).unwrap();
		assert_eq!(output, 'j');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn fail_not_consuming() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("kello".chars(), None);
		let parse_option = option(&parser1, &parser2);
		assert_eq!(
			parse_option(&mut input),
			Err(Error::UnexpectedToken(None, Position::new(1, 1), None))
		);
		assert_eq!(any()(&mut input), Ok('k')); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn fail_consuming() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let consuming_parser = |input: &mut Input<_>| {
			parser1(input)?;
			parser2(input)
		};
		let parse_option = option(&consuming_parser, &parser2);
		let output = parse_option(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 2), None))
		);
		assert_eq!(any()(&mut input), Ok('e')); // Ensure that the input was consumed.
	}
}
