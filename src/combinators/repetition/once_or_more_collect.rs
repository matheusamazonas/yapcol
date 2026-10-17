use super::core::repeat_no_end_collect;
use crate::{InputToken, Parser};

/// Applies `parser` one or more times, collecting all the matches.
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
/// instead in how many times it matched, consider using [`crate::once_or_more`].
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::once_or_more_collect`].
///
/// # Arguments
///
/// - `parser`: the parser to be applied one or more times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, once_or_more_collect};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), String::from("doctest.txt"));
/// assert_eq!(
/// 	once_or_more_collect(&parser)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Fails when no matches are found
/// let mut input = Input::new_from_chars("23".chars(), String::from("doctest.txt"));
/// assert!(once_or_more_collect(&parser)(&mut input).is_err());
///
/// // Fails on empty input
/// let mut input = Input::new_from_chars("".chars(), String::from("doctest.txt"));
/// assert!(once_or_more_collect(&parser)(&mut input).is_err());
/// ```
pub fn once_or_more_collect<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let (matches, _) = repeat_no_end_collect(parser, 1, None)(input)?;
		Ok(matches)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), test_utils::test_source_name());
		let parser_many1 = once_or_more_collect(&parser);
		assert_eq!(
			parser_many1(&mut input),
			Err(Error::EndOfInput(input.source_name(), Some(Box::new('h'))))
		);
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').once_or_more_collect();
		let mut input = Input::new_from_chars("".chars(), test_utils::test_source_name());
		assert_eq!(
			parser(&mut input),
			Err(Error::EndOfInput(input.source_name(), Some(Box::new('h'))))
		);
	}

	#[test]
	fn no_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jklmno".chars(), test_utils::test_source_name());
		let parser_many1 = once_or_more_collect(&parser);
		let mismatch = Mismatch::new('h', 'j');
		assert_eq!(
			parser_many1(&mut input),
			Err(Error::UnexpectedToken(
				input.source_name(),
				Position::new(1, 1),
				Some(mismatch)
			))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn no_match_shortcut() {
		let parser = is('h').once_or_more_collect();
		let mut input = Input::new_from_chars("jklmno".chars(), test_utils::test_source_name());
		let mismatch = Mismatch::new('h', 'j');
		assert_eq!(
			parser(&mut input),
			Err(Error::UnexpectedToken(
				input.source_name(),
				Position::new(1, 1),
				Some(mismatch)
			))
		);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn one_match() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hallo".chars(), test_utils::test_source_name());
		let parser_many1 = once_or_more_collect(&parser);
		let output = parser_many1(&mut input).unwrap();
		assert_eq!(output, vec!['h']);
		assert_eq!(input.consumed_count(), 1);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn one_match_shortcut() {
		let parser = is('h').once_or_more_collect();
		let mut input = Input::new_from_chars("hallo".chars(), test_utils::test_source_name());
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
		let mut input = Input::new_from_chars(tokens, test_utils::test_source_name());
		let parser_many1 = once_or_more_collect(&parser);
		let output = parser_many1(&mut input).unwrap();
		assert_eq!(output.len(), token_count);
		assert!(output.iter().all(|x| *x == 'h'));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn partial_match_then_stop() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhjklmnop".chars(), test_utils::test_source_name());
		let parser_many1 = once_or_more_collect(&parser);
		let output = parser_many1(&mut input).unwrap();
		assert_eq!(output, vec!['h', 'h']);
		assert_eq!(input.consumed_count(), 2);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		assert_eq!(any()(&mut input), Ok('j'));
	}

	#[test]
	fn non_consuming_parser_does_not_loop() {
		let parser = success(1); // Non-consuming parser.
		let mut input = Input::new_from_chars("hello".chars(), test_utils::test_source_name());
		let parser = parser.once_or_more_collect();
		let output = parser(&mut input);
		let position = Position::new(1, 1);
		assert_eq!(
			output,
			Err(Error::NonConsumingLoop(input.source_name(), position))
		);
	}

	#[test]
	fn match_consuming_upon_failure() {
		// Parser that consumes input upon failure:
		let parser = |input: &mut StringInput| {
			let o1 = is('#')(input)?;
			let o2 = is('a')(input)?;
			Ok((o1, o2))
		};
		let mut input = Input::new_from_chars("#a#e".chars(), test_utils::test_source_name());
		let many_parser = parser.once_or_more_collect();
		let output = many_parser(&mut input).unwrap();
		assert_eq!(output.len(), 1);
		assert_eq!(output[0], ('#', 'a'));
		assert_eq!(input.consumed_count(), 3); // The second attempt failed while consuming.
		assert_eq!(any()(&mut input), Ok('e'));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}
}
