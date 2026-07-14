use super::core::repeat_no_end_collect;
use crate::{InputToken, Parser};

/// Applies `parser` between 1 and a given number of times, ensuring that no more matches occur and
/// collecting all the matches.
///
/// # Outcome
///
/// This combinator succeeds if the argument parser succeeds between 1 and up to (and including)
/// `max_count` times. In that case, it returns a vector of matches of its argument parser.
///
/// It fails if the argument parser:
/// - Never succeeds.
/// - Matches more than `max_count` times.
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
/// # Performance
///
/// This combinator stores all the matches it finds. If you're not interested in the matches, but
/// instead in how many times it matched, consider using [`crate::once_up_to`].
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::once_up_to_collect`].
///
/// # Arguments
///
/// - `parser`: the parser to be applied one or more times.
/// - `max_count`: the (inclusive) maximum number of times that the argument parser should succeed.
///   Must be greater than 0, otherwise this function panics.
///
/// # Panics
///
/// This function panics if `max_count` is equal to 0. Check [`crate::up_to_collect`] if you would
/// like to cover this case.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, once_up_to_collect};
///
/// // Succeeds if the parser matches exactly `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 2;
/// assert_eq!(
/// 	once_up_to_collect(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Succeeds if the parser matches less than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 5;
/// assert_eq!(
/// 	once_up_to_collect(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Fails if the parser matches more than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// let max_count = 2;
/// assert!(once_up_to_collect(&parser, max_count)(&mut input).is_err());
/// ```
pub fn once_up_to_collect<P, IT, O>(parser: &P, max_count: usize) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	if max_count == 0 {
		panic!("max_count must be greater than 0");
	}
	move |input| {
		let (count, _) = repeat_no_end_collect(parser, 1, Some(max_count))(input)?;
		Ok(count)
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
		let parser_up_to = once_up_to_collect(&parser, 1);
		assert_eq!(
			parser_up_to(&mut input),
			Err(Error::EndOfInput(Some(Box::new('h'))))
		);
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').once_up_to_collect(1);
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
		let parser_up_to = once_up_to_collect(&parser, 1);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 1);
		assert_unexpected_error(output, position, "h", "j");
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn no_match_shortcut() {
		let parser = is('h').once_up_to_collect(1);
		let mut input = Input::new_from_chars("jklmno".chars(), None);
		let output = parser(&mut input);
		let position = Position::new(1, 1);
		assert_unexpected_error(output, position, "h", "j");
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn one_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_up_to = once_up_to_collect(&parser, 1);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, vec!['h']);
		assert_eq!(input.consumed_count(), 1);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn less_than_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhello".chars(), None);
		let parser_up_to = once_up_to_collect(&parser, 3);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, vec!['h', 'h']);
		assert_eq!(input.consumed_count(), 2);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn equal_to_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhello".chars(), None);
		let parser_up_to = once_up_to_collect(&parser, 3);
		let output = parser_up_to(&mut input).unwrap();
		assert_eq!(output, vec!['h', 'h', 'h']);
		assert_eq!(input.consumed_count(), 3);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('e'));
	}

	#[test]
	fn more_than_max_count() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhhello".chars(), None);
		let parser_up_to = once_up_to_collect(&parser, 3);
		let output = parser_up_to(&mut input);
		let position = Position::new(1, 4);
		assert_unexpected_error(output, position, "3", "4");
	}

	#[test]
	#[should_panic]
	fn zero_panics() {
		let parser = is::<CharToken>('h');
		let _ = once_up_to_collect(&parser, 0);
	}
}
