use super::core::{CountAccumulator, RepetitionAccumulator, repeat_no_end};
use crate::{InputToken, Parser};

/// Applies `parser` between 0 and a given number of times, ensuring that no more matches occur.
///
/// # Outcome
///
/// This combinator succeeds if the argument parser succeeds between 0 and up to (and including)
/// `max_count` times. Unlike [`crate::up_to_collect`], this combinator doesn't return its matches,
/// but just how many times it matched.
///
/// It fails if the argument parser matches more than `max_count` times.
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
/// This combinator has a shortcut version: [`Parser::up_to`].
///
/// # Arguments
///
/// - `parser`: The parser to be applied multiple times.
/// - `max_count`: The (inclusive) maximum number of times that the argument parser should succeed.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, up_to};
///
/// // Succeeds if the parser matches exactly `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 5;
/// assert_eq!(up_to(&parser, max_count)(&mut input), Ok(2));
///
/// // Succeeds if the parser matches less than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 5;
/// assert_eq!(up_to(&parser, max_count)(&mut input), Ok(2));
///
/// // Fails if the parser matches more than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// let max_count = 2;
/// assert!(up_to(&parser, max_count)(&mut input).is_err());
///
/// // Succeeds on empty input if `max_count` is 0.
/// let mut input = Input::new_from_chars("".chars(), None);
/// let max_count = 0;
/// assert_eq!(up_to(&parser, max_count)(&mut input), Ok(0));
/// ```
pub fn up_to<P, IT, O>(parser: &P, max_count: usize) -> impl Parser<IT, usize>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let accumulator: CountAccumulator<O> = repeat_no_end(parser, 0, Some(max_count))(input)?;
		Ok(accumulator.value())
	}
}

#[cfg(test)]
mod tests {
	use crate::combinators::repetition::test_utils::assert_unexpected_error;
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let parser_up_to = up_to(&parser, 1);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 0);
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').up_to(1);
		let mut input = Input::new_from_chars("".chars(), None);
		let output = parser(&mut input).unwrap();
		assert_eq!(output, 0);
	}

	#[test]
	fn no_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jklmno".chars(), None);
		let parser_up_to = up_to(&parser, 1);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 0);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn no_match_shortcut() {
		let parser = is('h').up_to(1);
		let mut input = Input::new_from_chars("jklmno".chars(), None);
		let output = parser(&mut input).unwrap();
		assert_eq!(output, 0);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn one_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_up_to = up_to(&parser, 1);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 1);
		assert_eq!(input.consumed_count(), 1);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn less_than_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhello".chars(), None);
		let parser_up_to = up_to(&parser, 3);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 2);
		assert_eq!(input.consumed_count(), 2);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn equal_to_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhello".chars(), None);
		let parser_up_to = up_to(&parser, 3);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 3);
		assert_eq!(input.consumed_count(), 3);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn more_than_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhhello".chars(), None);
		let parser_up_to = up_to(&parser, 3);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 4);
		assert_unexpected_error(output, position, "3", "4");
	}

	#[test]
	fn zero_empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let parser_up_to = up_to(&parser, 0);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 0);
		assert_eq!(input.consumed_count(), 0);
	}

	#[test]
	fn zero_success() {
		let parser = is('h');
		let mut input = Input::new_from_chars("ello".chars(), None);
		let parser_up_to = up_to(&parser, 0);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, 0);
		assert_eq!(input.consumed_count(), 0);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn zero_fail() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_up_to = up_to(&parser, 0);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 1);
		assert_unexpected_error(output, position, "0", "1");
		assert_eq!(input.consumed_count(), 1);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}
}
