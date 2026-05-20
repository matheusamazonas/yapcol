use crate::{Error, Input, InputToken, Parser};

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
/// - At least one occurrence of its argument parser succeeds, *and* its argument parser consumes
///   input upon success.
/// - If its argument parser consumes input upon failure, independent of this combinator's outcome.
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
	|input| {
		let output: Vec<O> = Vec::new();
		many(parser)(input, output)
	}
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
/// - At least one occurrence of its argument parser succeeds, *and* its argument parser consumes
///   input upon success.
/// - If its argument parser consumes input upon failure, independent of this combinator's outcome.
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
	|input| {
		let mut output: Vec<O> = Vec::new();
		match parser(input) {
			Ok(token) => {
				output.push(token);
				many(parser)(input, output)
			}
			Err(e) => Err(e),
		}
	}
}

fn many<P, IT, O>(parser: &P) -> impl Fn(&mut Input<IT>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input, mut output| {
		let mut previous_count: Option<usize> = None;
		loop {
			match parser(input) {
				Ok(token) => {
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
					output.push(token);
					previous_count = Some(new_count);
				}
				Err(_) => return Ok(output),
			}
		}
	}
}

#[cfg(test)]
mod tests {
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
			assert_eq!(output.len(), 1);
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("hallo".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 1);
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
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
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
}
