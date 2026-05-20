use crate::{Error, InputToken, Parser};

/// Parses zero or more instances of `parser`, until `end` succeeds.
///
/// # Outcome
///
/// If it succeeds, this combinator returns a vector of matches of its `parser` argument.
///
/// # Input consumption
///
/// This parser consumes input if:
/// - It succeeds, and:
///   - Its `end` argument parser consumes upon success, or;
///   - Its `parser` argument consumes upon success, *and* it was applied at least once.
/// - It fails, and:
///   - Its `end` argument parser consumes upon failure, or;
///   - Its `parser` argument consumes upon failure, *and* it was applied at least once.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Arguments
///
/// - `parser`: the parser for the elements to be collected until the end is reached.
/// - `end`: the parser that delimits the end.
///
/// # Examples
///
/// ```
/// use yapcol::{Error, Input, any, is, many_until};
///
/// let comments_parser = |input: &mut Input<_>| {
/// 	let open = is('#');
/// 	let close = is('$');
/// 	let any = any();
/// 	open(input)?;
/// 	many_until(&any, &close)(input)
/// };
/// let mut input = Input::new_from_chars("#this is a comment$".chars(), None);
/// let output = comments_parser(&mut input);
/// assert_eq!(output, Ok("this is a comment".chars().collect()));
/// ```
pub fn many_until<P, PE, IT, O, OE>(parser: &P, end: &PE) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
{
	|input| {
		let mut matches = Vec::new();
		let mut previous_count: Option<usize> = None;
		while end(input).is_err() {
			let token = parser(input)?;
			let new_count = input.consumed_count();
			// Check if non-consuming parser. If so, it would cause an infinite loop.
			if let Some(previous) = previous_count
				&& previous == new_count
			{
				return Err(Error::NonConsumingLoop(
					input.source_name(),
					input.position(),
				));
			}

			matches.push(token);
			previous_count = Some(new_count);
		}
		Ok(matches)
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(None)));
	}

	#[test]
	fn success_none() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("#".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input).unwrap();
		assert_eq!(output, Vec::<char>::new());
	}

	#[test]
	fn success_multiple() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("Hello world #".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input).unwrap();
		assert_eq!(output, "Hello world ".chars().collect::<Vec<_>>());
	}

	#[test]
	fn fail() {
		let any_parser = is('x');
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("xxxxxy".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
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
		let parser = many_until(&non_consuming, &end_parser);
		let output = parser(&mut input);
		let position = Position::new(1, 1);
		assert_eq!(output, Err(Error::NonConsumingLoop(None, position)));
	}
}
