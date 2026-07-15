use super::core::repeat_with_end_collect;
use crate::{InputToken, Parser};

/// Parses zero or more instances of `parser`, until `end` succeeds, collecting all the matches.
///
/// # Outcome
///
/// If it succeeds, this combinator returns a tuple containing:
///  - A vector of matches of its `parser` argument.
///  - The match of its `end` argument parser.
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
/// # Performance
///
/// This combinator stores all the matches it finds. If you're not interested in the matches, but
/// instead in how many times it matched, consider using [`crate::many_until`].
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::many_until_collect`].
///
/// # Arguments
///
/// - `parser`: the parser for the elements to be collected until the end is reached.
/// - `end`: the parser that delimits the end.
///
/// # Examples
///
/// ```
/// use yapcol::{Error, Input, any, is, many_until_collect};
///
/// let comments_parser = |input: &mut Input<_>| {
/// 	let open = is('#');
/// 	let close = is('$');
/// 	let any = any();
/// 	open(input)?;
/// 	many_until_collect(&any, &close)(input)
/// };
/// let mut input = Input::new_from_chars("#this is a comment$".chars(), None);
/// let (matches, end) = comments_parser(&mut input).unwrap();
/// assert_eq!(matches, "this is a comment".chars().collect::<Vec<char>>());
/// assert_eq!(end, '$');
/// ```
pub fn many_until_collect<P, PE, IT, O, OE>(parser: &P, end: &PE) -> impl Parser<IT, (Vec<O>, OE)>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
{
	move |input| {
		let (count, end) = repeat_with_end_collect(parser, 0, None, end)(input)?;
		Ok((count, end.expect("End parser value is missing.")))
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
		let many_until_collect_parser = many_until_collect(&any_parser, &end_comment_parser);
		let output = many_until_collect_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(None)));
	}

	#[test]
	fn no_matches_succeeds() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("#".chars(), None);
		let many_until_collect_parser = many_until_collect(&any_parser, &end_comment_parser);
		let (matches, end) = many_until_collect_parser(&mut input).unwrap();
		assert_eq!(matches, Vec::<char>::new());
		assert_eq!(end, '#');
	}

	#[test]
	fn multiple_matches_succeeds() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("Hello world #".chars(), None);
		let many_until_collect_parser = many_until_collect(&any_parser, &end_comment_parser);
		let (matches, end) = many_until_collect_parser(&mut input).unwrap();
		assert_eq!(matches, "Hello world ".chars().collect::<Vec<_>>());
		assert_eq!(end, '#');
	}

	#[test]
	fn no_end_fails() {
		let any_parser = is('x');
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("xxxxxy".chars(), None);
		let many_until_collect_parser = many_until_collect(&any_parser, &end_comment_parser);
		let output = many_until_collect_parser(&mut input);
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
		let many_until_collect_parser = many_until_collect(&non_consuming, &end_parser);
		let output = many_until_collect_parser(&mut input);
		let position = Position::new(1, 1);
		assert_eq!(output, Err(Error::NonConsumingLoop(None, position)));
	}
}
