use crate::{InputToken, Parser};

/// Creates a parser based on two input parsers. It tries the first parser and falls back to the
/// second if the first fails without consuming input.
///
/// # Outcome
///
/// This combinator succeeds if:
/// - The first parser argument succeeds. In this case, the second parser argument isn't applied.
/// - The first parser argument fails without consuming any input, and the second parser argument
///   succeeds.
///
/// It fails if:
/// - The first parser argument fails while consuming input. In this case, the second parser
///   argument isn't even applied.
/// - The first parser argument fails without consuming input, and the second parser argument fails.
///
/// # Input consumption
///
/// This combinator consumes input if:
/// - It succeeds, and the succeeding argument parser consumes input upon success.
/// - Any of its argument parsers fail while consuming input.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure. Check the
/// [`crate::attempt`] combinator if you would like to change that behavior.
///
/// # Arguments
///
/// - `parser1`: The first parser to try.
/// - `parser2`: The fallback parser, applied only if `parser1` fails without consuming input.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, either, end_of_input, is};
///
/// // parser1 succeeds: returns its result.
/// let mut input = Input::new_from_chars("ab".chars(), None);
/// let output = either(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'a');
///
/// // parser1 fails without consuming input: parser2 is tried.
/// let mut input = Input::new_from_chars("b".chars(), None);
/// let output = either(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'b');
///
/// // Both parsers fail: an error is returned and input is not consumed.
/// let mut input = Input::new_from_chars("c".chars(), None);
/// assert!(either(&is('a'), &is('b'))(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('c'));
/// ```
pub fn either<P1, P2, IT, O>(parser1: &P1, parser2: &P2) -> impl Parser<IT, O>
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
		let parse_either = either(&parser1, &parser2);
		let output = parse_either(&mut input).unwrap();
		assert_eq!(output, 'h');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn success_second() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("j".chars(), None);
		let parse_either = either(&parser1, &parser2);
		let output = parse_either(&mut input).unwrap();
		assert_eq!(output, 'j');
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn fail_not_consuming() {
		let parser1 = is('h');
		let parser2 = is('j');
		let mut input = Input::new_from_chars("kello".chars(), None);
		let parse_either = either(&parser1, &parser2);
		let mismatch = Mismatch::new('j', 'k');
		assert_eq!(
			parse_either(&mut input),
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
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
		let parse_either = either(&consuming_parser, &parser2);
		let output = parse_either(&mut input);
		let mismatch = Mismatch::new('j', 'e');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
		assert_eq!(any()(&mut input), Ok('e')); // Ensure that the input was consumed.
	}
}
