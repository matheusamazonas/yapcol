use super::core::{CountAccumulator, RepetitionAccumulator, repeat_with_end};
use crate::{InputToken, Parser};

/// Applies `parser` one or more times, until `end` succeeds.
///
/// # Outcome
///
/// If successful, unlike [`crate::once_or_more_until_collect`], this combinator doesn't return
/// its matches, but just how many times it matched.
///
/// # Input consumption
///
/// This parser consumes input if:
/// - It succeeds, and:
///   - Its `end` argument parser consumes upon success, or;
///   - Its `parser` argument succeeded at least once.
/// - It fails, and:
///   - Its `end` argument parser consumes upon failure, or;
///   - Its `parser` argument consumes upon failure, *and* it was applied at least once.
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
/// This combinator has a shortcut version: [`Parser::once_or_more_until`].
///
/// # Arguments
///
/// - `parser`: the parser to be applied one or more times.
/// - `end`: the parser that delimits the end.
///
/// # Examples
///
/// ```
/// use yapcol::{Error, Input, any, is, once_or_more_until};
///
/// let comments_parser = |input: &mut Input<_>| {
/// 	let open = is('#');
/// 	let close = is('$');
/// 	let any = any();
/// 	open(input)?;
/// 	once_or_more_until(&any, &close)(input)
/// };
/// // Success if there is at least one character before the end.
/// let mut input = Input::new_from_chars("#12345$".chars(), None);
/// let output = comments_parser(&mut input);
/// assert_eq!(output, Ok(5));
///
/// // Fails if there is no character before the end.
/// let mut input = Input::new_from_chars("#$".chars(), None);
/// let output = comments_parser(&mut input);
/// assert!(output.is_err());
/// ```
pub fn once_or_more_until<P, PE, IT, O, OE>(parser: &P, end: &PE) -> impl Parser<IT, usize>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
{
	move |input| {
		let accumulator: CountAccumulator<O> = repeat_with_end(parser, 1, None, end)(input)?;
		Ok(accumulator.value())
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty_fails() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("".chars(), None);
		let not_followed_parser = once_or_more_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(None)));
	}

	#[test]
	fn no_match_fails() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("#".chars(), None);
		let not_followed_parser = once_or_more_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
		let mismatch = Mismatch::new("at least 1 occurrences", "0 occurrences");
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
		);
	}

	#[test]
	fn multiple_matches_succeeds() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("123456#".chars(), None);
		let many_parser = once_or_more_until(&any_parser, &end_comment_parser);
		let output = many_parser(&mut input).unwrap();
		assert_eq!(output, 6);
	}

	#[test]
	fn no_end_fails() {
		let any_parser = is('x');
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("xxxxxy".chars(), None);
		let many_parser = once_or_more_until(&any_parser, &end_comment_parser);
		let output = many_parser(&mut input);
		let mismatch = Mismatch::new('x', 'y');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 6),
				Some(mismatch)
			))
		);
		assert_eq!(any()(&mut input), Ok('y')); // Input was consumed while looking for the end.
	}

	#[test]
	fn non_consuming_parser_does_not_loop() {
		let non_consuming = success(1); // Non-consuming parser.
		let mut input = Input::new_from_chars("hello#".chars(), None);
		let end_parser = is('#');
		let parser = once_or_more_until(&non_consuming, &end_parser);
		let output = parser(&mut input);
		let position = Position::new(1, 1);
		assert_eq!(output, Err(Error::NonConsumingLoop(None, position)));
	}
}
