use crate::{Error, InputToken, Mismatch, Parser};

/// Applies `parser` zero or more times.
///
/// # Outcome
///
/// This parser always succeeds, even if its argument parser doesn't. It returns a vector of
/// matches of its argument parser, which might be empty in case no matches were found.
///
/// # Input consumption
///
/// This parser consumes input if:
/// - At least one occurrence of its argument parser succeeds.
/// - If its argument parser consumes input upon failure, independent of this combinator's outcome.
///
/// # Error handling
///
/// This combinator fails with [`Error::NonConsumingLoop`] if the argument parser does not consume
/// input upon success. This behavior is there to prevent an infinite loop caused by the input never
/// being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead. It also never backtracks, given that it never
/// fails.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many0`].
///
/// # Arguments
///
/// - `parser`: The parser to possibly be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many0};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok("11".chars().collect()));
///
/// // Returns an empty vector when no matches are found (never fails)
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
///
/// // Returns an empty vector on empty input (never fails)
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
/// ```
pub fn many0<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	many(parser, 0, None)
}

/// Applies `parser` one or more times.
///
/// # Outcome
///
/// If it succeeds, this combinator returns a (non-empty) vector of matches of its argument parser.
///
/// # Input consumption
///
/// This parser consumes input if:
/// - At least one occurrence of its argument parser succeeds.
/// - If its argument parser consumes input upon failure, independent of this combinator's outcome.
///
/// # Error handling
///
/// This combinator fails with [`Error::NonConsumingLoop`] if the argument parser does not consume
/// input upon success. This behavior is there to prevent an infinite loop caused by the input never
/// being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many1`].
///
/// # Arguments
///
/// - `parser`: The parser to be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many1};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// assert_eq!(many1(&parser)(&mut input), Ok("11".chars().collect()));
///
/// // Fails when no matches are found
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert!(many1(&parser)(&mut input).is_err());
///
/// // Fails on empty input
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(many1(&parser)(&mut input).is_err());
/// ```
pub fn many1<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	many(parser, 1, None)
}

/// Applies `parser` between 0 and a given number of times, ensuring that no more matches occur.
///
/// # Outcome
///
/// This combinator succeeds if the argument parser succeeds between 0 and up to (and including)
/// `max_count` times. In that case, it returns a vector of matches of its argument parser.
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
/// This combinator fails with [`Error::NonConsumingLoop`] if the argument parser does not consume
/// input upon success. This behavior is there to prevent an infinite loop caused by the input never
/// being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many0_up_to`].
///
/// # Arguments
///
/// - `parser`: The parser to be applied multiple times.
/// - `max_count`: The (inclusive) maximum number of times that the argument parser should succeed.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many0_up_to};
///
/// // Succeeds if the parser matches exactly `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 2;
/// assert_eq!(
/// 	many0_up_to(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Succeeds if the parser matches less than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 5;
/// assert_eq!(
/// 	many0_up_to(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Fails if the parser matches more than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// let max_count = 2;
/// assert!(many0_up_to(&parser, max_count)(&mut input).is_err());
///
/// // Succeeds on empty input if `max_count` is 0.
/// let mut input = Input::new_from_chars("".chars(), None);
/// let max_count = 0;
/// assert_eq!(many0_up_to(&parser, max_count)(&mut input), Ok(Vec::new()));
/// ```
pub fn many0_up_to<P, IT, O>(parser: &P, max_count: usize) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	many(parser, 0, Some(max_count))
}

/// Applies `parser` between 1 and a given number of times, ensuring that no more matches occur.
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
/// This combinator fails with [`Error::NonConsumingLoop`] if the argument parser does not consume
/// input upon success. This behavior is there to prevent an infinite loop caused by the input never
/// being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many1_up_to`].
///
/// # Arguments
///
/// - `parser`: The parser to be applied multiple times.
/// - `max_count`: The (inclusive) maximum number of times that the argument parser should succeed.
///   Must be greater than 0, otherwise this function panics.
///
/// # Panics
///
/// This function panics if `max_count` is equal to 0. Check [`many0_up_to`] if you would like to
/// cover this case.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many1_up_to};
///
/// // Succeeds if the parser matches exactly `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 2;
/// assert_eq!(
/// 	many1_up_to(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Succeeds if the parser matches less than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// let max_count = 5;
/// assert_eq!(
/// 	many1_up_to(&parser, max_count)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Fails if the parser matches more than `max_count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// let max_count = 2;
/// assert!(many1_up_to(&parser, max_count)(&mut input).is_err());
/// ```
pub fn many1_up_to<P, IT, O>(parser: &P, max_count: usize) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	if max_count == 0 {
		panic!("max_count must be greater than 0");
	}
	many(parser, 1, Some(max_count))
}

fn many<P, IT, O>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let mut matches: Vec<O> = Vec::new();
		let mut total_match_count = 0;
		let mut previous_consumed_count = input.consumed_count();
		loop {
			let previous_position = input.position();
			let outcome = parser(input);
			match (outcome, max_match_count) {
				// Matched too many times.
				(Ok(_), Some(max_count)) if max_count == total_match_count => {
					total_match_count += 1;
					let expected = format!("at most {max_count} occurrences");
					let found = format!("{total_match_count} occurrences");
					let mismatch = Mismatch::new(expected, found);
					return Err(Error::UnexpectedToken(
						input.source_name(),
						previous_position,
						Some(mismatch),
					));
				}
				// Valid match.
				(Ok(token), _) => {
					total_match_count += 1;
					let consumed_count = input.consumed_count();
					// Check if non-consuming parser. If so, it would cause an infinite loop.
					if previous_consumed_count == consumed_count {
						return Err(Error::NonConsumingLoop(
							input.source_name(),
							input.position(),
						));
					}
					matches.push(token);
					previous_consumed_count = consumed_count;
				}
				(Err(e), _) => {
					return if total_match_count >= min_match_count {
						Ok(matches)
					} else {
						Err(e)
					};
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Error;
	use crate::input::Position;
	use std::fmt::Debug;

	fn assert_unexpected_error<T>(
		value: Result<T, Error>,
		position: Position,
		expected: &str,
		found: &str,
	) where
		T: Debug,
	{
		let error = value.unwrap_err();
		if let Error::UnexpectedToken(_, error_pos, mismatch) = error {
			if error_pos != position {
				panic!("Expected error position to be {position}, but got {error_pos}");
			}
			let mismatch_message = mismatch.unwrap().to_string();
			let mut split = mismatch_message.split("found:");
			let expected_message = split.next().unwrap();
			assert!(expected_message.contains(expected));
			let found_message = split.next().unwrap();
			assert!(found_message.contains(found));
		} else {
			panic!(
				"Expected error to be of type UnexpectedToken, but got {:?}",
				error
			);
		}
	}

	mod many0 {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many0();
			let mut input = Input::new_from_chars("".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn no_match_not_empty() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('j', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_not_empty_shortcut() {
			let token_count = 100;
			let parser = is('h').many0();
			let tokens = std::iter::repeat_n('j', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn match_not_empty() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert_eq!(input.consumed_count(), token_count);
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn match_not_empty_shortcut() {
			let token_count = 100;
			let parser = is('h').many0();
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert_eq!(input.consumed_count(), token_count);
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn partial_match_then_stop() {
			let parser = is('#');
			let mut input = Input::new_from_chars("#####Hello".chars(), None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output, "#####".chars().collect::<Vec<_>>());
			assert_eq!(input.consumed_count(), 5);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
			assert_eq!(any()(&mut input), Ok('H'));
		}

		#[test]
		fn non_consuming_parser_does_not_loop() {
			let parser = success(1); // Non-consuming parser.
			let mut input = Input::new_from_chars("hello".chars(), None);
			let parser = parser.many0();
			let output = parser(&mut input);
			let position = Position::new(1, 1);
			assert_eq!(output, Err(Error::NonConsumingLoop(None, position)));
		}

		#[test]
		fn match_consuming_upon_failure() {
			// Parser that consumes input upon failure:
			let parser = |input: &mut StringInput| {
				let o1 = is('#')(input)?;
				let o2 = is('a')(input)?;
				Ok((o1, o2))
			};
			let mut input = Input::new_from_chars("#a#e".chars(), None);
			let many_parser = parser.many0();
			let output = many_parser(&mut input).unwrap();
			assert_eq!(output.len(), 1);
			assert_eq!(output[0], ('#', 'a'));
			assert_eq!(input.consumed_count(), 3); // The second attempt failed while consuming.
			assert_eq!(any()(&mut input), Ok('e'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}
	}

	mod many1 {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_many1 = many1(&parser);
			assert_eq!(
				parser_many1(&mut input),
				Err(Error::EndOfInput(Some(Box::new('h'))))
			);
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many1();
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
			let parser_many1 = many1(&parser);
			let mismatch = Mismatch::new('h', 'j');
			assert_eq!(
				parser_many1(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("jklmno".chars(), None);
			let mismatch = Mismatch::new('h', 'j');
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hallo".chars(), None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output, vec!['h']);
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("hallo".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output, vec!['h']);
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn multiple_matches() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert!(output.iter().all(|x| *x == 'h'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn partial_match_then_stop() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hhjklmnop".chars(), None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output, vec!['h', 'h']);
			assert_eq!(input.consumed_count(), 2);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
			assert_eq!(any()(&mut input), Ok('j'));
		}

		#[test]
		fn non_consuming_parser_does_not_loop() {
			let parser = success(1); // Non-consuming parser.
			let mut input = Input::new_from_chars("hello".chars(), None);
			let parser = parser.many1();
			let output = parser(&mut input);
			let position = Position::new(1, 1);
			assert_eq!(output, Err(Error::NonConsumingLoop(None, position)));
		}

		#[test]
		fn match_consuming_upon_failure() {
			// Parser that consumes input upon failure:
			let parser = |input: &mut StringInput| {
				let o1 = is('#')(input)?;
				let o2 = is('a')(input)?;
				Ok((o1, o2))
			};
			let mut input = Input::new_from_chars("#a#e".chars(), None);
			let many_parser = parser.many1();
			let output = many_parser(&mut input).unwrap();
			assert_eq!(output.len(), 1);
			assert_eq!(output[0], ('#', 'a'));
			assert_eq!(input.consumed_count(), 3); // The second attempt failed while consuming.
			assert_eq!(any()(&mut input), Ok('e'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}
	}

	mod many0_up_to {
		use super::assert_unexpected_error;
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_up_to = many0_up_to(&parser, 1);
			let output = parser_up_to(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many0_up_to(1);
			let mut input = Input::new_from_chars("".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn no_match() {
			let parser = is('h');
			let mut input = Input::new_from_chars("jklmno".chars(), None);
			let parser_up_to = many0_up_to(&parser, 1);
			let output = parser_up_to(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_shortcut() {
			let parser = is('h').many0_up_to(1);
			let mut input = Input::new_from_chars("jklmno".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hello".chars(), None);
			let parser_up_to = many0_up_to(&parser, 1);
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
			let parser_up_to = many0_up_to(&parser, 3);
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
			let parser_up_to = many0_up_to(&parser, 3);
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
			let parser_up_to = many0_up_to(&parser, 3);
			let output = parser_up_to(&mut input);
			let position = Position::new(1, 4);
			assert_unexpected_error(output, position, "3", "4");
		}

		#[test]
		fn zero_empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_up_to = many0_up_to(&parser, 0);
			let output = parser_up_to(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
		}

		#[test]
		fn zero_success() {
			let parser = is('h');
			let mut input = Input::new_from_chars("ello".chars(), None);
			let parser_up_to = many0_up_to(&parser, 0);
			let output = parser_up_to(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
			assert_eq!(any()(&mut input), Ok('e'));
		}

		#[test]
		fn zero_fail() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hello".chars(), None);
			let parser_up_to = many0_up_to(&parser, 0);
			let output = parser_up_to(&mut input);
			let position = Position::new(1, 1);
			assert_unexpected_error(output, position, "0", "1");
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
			assert_eq!(any()(&mut input), Ok('e'));
		}
	}

	mod many1_up_to {
		use super::assert_unexpected_error;
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_up_to = many1_up_to(&parser, 1);
			assert_eq!(
				parser_up_to(&mut input),
				Err(Error::EndOfInput(Some(Box::new('h'))))
			);
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many1_up_to(1);
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
			let parser_up_to = many1_up_to(&parser, 1);
			let output = parser_up_to(&mut input);
			let position = Position::new(1, 1);
			assert_unexpected_error(output, position, "h", "j");
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_shortcut() {
			let parser = is('h').many1_up_to(1);
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
			let parser_up_to = many1_up_to(&parser, 1);
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
			let parser_up_to = many1_up_to(&parser, 3);
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
			let parser_up_to = many1_up_to(&parser, 3);
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
			let parser_up_to = many1_up_to(&parser, 3);
			let output = parser_up_to(&mut input);
			let position = Position::new(1, 4);
			assert_unexpected_error(output, position, "3", "4");
		}

		#[test]
		#[should_panic]
		fn zero_panics() {
			let parser = is::<CharToken>('h');
			let _ = many1_up_to(&parser, 0);
		}
	}
}
