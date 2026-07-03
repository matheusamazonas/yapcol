use super::core::{MatchesAccumulator, RepetitionAccumulator, repeat_no_end};
use crate::{InputToken, Parser};

/// Applies `parser` at least a given number of times (and possibly more), collecting the matches.
///
/// # Outcome
///
/// This combinator succeeds if the argument parser succeeds at least `min_count` times. In that
/// case, it returns a vector of matches of its argument parser.
///
/// It fails if the argument parser matches fewer than `min_count` times.
///
/// # Input consumption
///
/// This parser consumes input if:
/// - At least one occurrence of its argument parser succeeds.
/// - If its argument parser consumes input upon failure, independent of this combinator's outcome.
///
/// # Error handling
///
/// This combinator fails with [`crate::Error::NonConsumingLoop`] if the argument parser does not
/// consume input upon success. This behavior is there to prevent an infinite loop caused by the
/// input never being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::at_least_collect`].
///
/// # Arguments
///
/// - `parser`: The parser to be applied multiple times.
/// - `min_count`: The minimum number of times that the argument parser should succeed.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, at_least_collect, is};
///
/// // Succeeds if the parser matches exactly `min_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let min_count = 2;
/// assert_eq!(
/// 	at_least_collect(&parser, min_count)(&mut input),
/// 	Ok(vec!['1', '1'])
/// );
///
/// // Succeeds if the parser matches more than `min_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// let min_count = 2;
/// assert_eq!(
/// 	at_least_collect(&parser, min_count)(&mut input),
/// 	Ok(vec!['1', '1', '1'])
/// );
///
/// // Fails if the parser matches more than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1".chars(), None);
/// let max_count = 2;
/// assert!(at_least_collect(&parser, max_count)(&mut input).is_err());
/// ```
pub fn at_least_collect<P, IT, O>(parser: &P, min_count: usize) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let accumulator: MatchesAccumulator<O> = repeat_no_end(parser, min_count, None)(input)?;
		Ok(accumulator.value())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::combinators::repetition::test_utils::assert_unexpected_error;
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let parser_up_to = at_least_collect(&parser, 1);
		assert_eq!(
			parser_up_to(&mut input),
			Err(Error::EndOfInput(Some(Box::new('h'))))
		);
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').at_least_collect(1);
		let mut input = Input::new_from_chars("".chars(), None);
		assert_eq!(
			parser(&mut input),
			Err(Error::EndOfInput(Some(Box::new('h'))))
		);
	}

	#[test]
	fn no_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jklmno".chars(), None);
		let parser_up_to = at_least_collect(&parser, 1);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 1);
		assert_unexpected_error(output, position, "h", "j");
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn no_match_shortcut() {
		let parser = is('h').at_least_collect(1);
		let mut input = Input::new_from_chars("jklmno".chars(), None);
		let output = parser(&mut input);
		let position = Position::new(1, 1);
		assert_unexpected_error(output, position, "h", "j");
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn exact_count_succeeds() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_up_to = at_least_collect(&parser, 1);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, vec!['h']);
		assert_eq!(input.consumed_count(), 1);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn more_than_count_succeeds() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhello".chars(), None);
		let parser_up_to = at_least_collect(&parser, 2);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, vec!['h', 'h', 'h']);
		assert_eq!(input.consumed_count(), 3);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn less_than_min_count_fails() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhello".chars(), None);
		let parser_up_to = at_least_collect(&parser, 3);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 3);
		assert_unexpected_error(output, position, "h", "e");
	}
}
