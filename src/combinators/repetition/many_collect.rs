use super::core::{ManyOutput, many_no_end};
use crate::{InputToken, Parser};

/// Applies `parser` zero or more times, collecting all the potential matches.
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
/// This combinator fails with [`crate::Error::NonConsumingLoop`] if the argument parser does not
/// consume input upon success. This behavior is there to prevent an infinite loop caused by the
/// input never being consumed.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead. It also never backtracks, given that it never
/// fails.
///
/// # Performance
///
/// This combinator stores all the matches it finds. If you're not interested in the matches, but
/// instead in how many times it matched, consider using [`crate::many`].
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many_collect`].
///
/// # Arguments
///
/// - `parser`: The parser to possibly be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many_collect};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// assert_eq!(
/// 	many_collect(&parser)(&mut input),
/// 	Ok("11".chars().collect())
/// );
///
/// // Returns an empty vector when no matches are found (never fails)
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert_eq!(many_collect(&parser)(&mut input), Ok(vec![]));
///
/// // Returns an empty vector on empty input (never fails)
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert_eq!(many_collect(&parser)(&mut input), Ok(vec![]));
/// ```
pub fn many_collect<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| match many_no_end(parser, 0, None, true)(input) {
		Ok(ManyOutput::Matches(matches)) => Ok(matches),
		Ok(ManyOutput::Count(_)) => panic!("Expected Matches, but got Count."),
		Err(e) => Err(e),
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
		let mut input = Input::new_from_chars("".chars(), None);
		let parser_many0 = many_collect(&parser);
		let output = parser_many0(&mut input).unwrap();
		assert_eq!(output.len(), 0);
	}

	#[test]
	fn empty_shortcut() {
		let parser = is('h').many_collect();
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
		let parser_many0 = many_collect(&parser);
		let output = parser_many0(&mut input).unwrap();
		assert_eq!(output.len(), 0);
		assert_eq!(input.consumed_count(), 0);
		assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
	}

	#[test]
	fn no_match_not_empty_shortcut() {
		let token_count = 100;
		let parser = is('h').many_collect();
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
		let parser_many0 = many_collect(&parser);
		let output = parser_many0(&mut input).unwrap();
		assert_eq!(output.len(), token_count);
		assert_eq!(input.consumed_count(), token_count);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn match_not_empty_shortcut() {
		let token_count = 100;
		let parser = is('h').many_collect();
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
		let parser_many0 = many_collect(&parser);
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
		let parser = parser.many_collect();
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
		let many_parser = parser.many_collect();
		let output = many_parser(&mut input).unwrap();
		assert_eq!(output.len(), 1);
		assert_eq!(output[0], ('#', 'a'));
		assert_eq!(input.consumed_count(), 3); // The second attempt failed while consuming.
		assert_eq!(any()(&mut input), Ok('e'));
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}
}
