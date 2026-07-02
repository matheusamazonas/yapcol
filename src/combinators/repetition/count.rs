use crate::combinators::repetition::core::{RepetitionOutput, repeat_no_end};
use crate::{InputToken, Parser};

/// Creates a parser that applies the given parser exactly `count` times.
///
/// # Outcome
///
/// If successful, unlike [`crate::count_collect`], this combinator doesn't return its matches,
/// but just how many times it matched.
///
/// This combinator succeeds if:
/// - `parser` occurs exactly `count` times in a row.
/// - Its `count` argument is 0.
///
/// It fails:
///  - The `parser` argument fails at any point before being applied `count` times.
///  - The `parser` argument succeeded more than `count` times.
///
/// # Input consumption
///
/// This combinator consumes input if:
/// - It succeeds with `count` greater than 0, and its argument parser consumes input upon success.
/// - It fails and its argument parser consumes input upon failure.
///
/// # Look-ahead and backtracking
///
/// This combinator doesn't perform any lookahead and won't backtrack upon failure.
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::count`].
///
/// # Arguments
///
/// - `parser`: The parser to apply repeatedly.
/// - `count`: The exact number of times to apply the parser.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, count, is};
///
/// // Succeeds when the parser matches exactly `count` times.
/// let parser = is('1');
/// let mut input = Input::new_from_chars("1112".chars(), None);
/// assert_eq!(count(&parser, 3)(&mut input), Ok(3));
/// assert_eq!(any()(&mut input), Ok('2')); // Remaining input after consuming 3 tokens.
///
/// // Fails when there are not enough matching tokens
/// let mut input = Input::new_from_chars("123".chars(), None);
/// assert!(count(&parser, 3)(&mut input).is_err());
///
/// // Fails on empty input when count > 0
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(count(&parser, 1)(&mut input).is_err());
/// ```
pub fn count<P, IT, O>(parser: &P, count: usize) -> impl Parser<IT, usize>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| match repeat_no_end(parser, count, Some(count), false)(input) {
		Ok(RepetitionOutput::Matches(_)) => panic!("Expected Count, but got Matches."),
		Ok(RepetitionOutput::Count(count)) => Ok(count),
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn count_zero_empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("".chars(), None);
		let parser = count(&parser, 0);
		let output = parser(&mut input);
		assert_eq!(output, Ok(0));
	}

	#[test]
	fn count_zero_empty_shortcut() {
		let parser = is('h').count(0);
		let mut input = Input::new_from_chars("".chars(), None);
		let output = parser(&mut input);
		assert_eq!(output, Ok(0));
	}

	#[test]
	fn count_0_not_empty_no_match_succeeds() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jello".chars(), None);
		let parser = count(&parser, 0);
		let output = parser(&mut input);
		assert_eq!(output, Ok(0));
	}

	#[test]
	fn count_0_not_empty_shortcut_no_match_succeeds() {
		let parser = is('h').count(0);
		let mut input = Input::new_from_chars("jello".chars(), None);
		let output = parser(&mut input);
		assert_eq!(output, Ok(0));
	}

	#[test]
	fn count_0_too_many_fails() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser = count(&parser, 0);
		let output = parser(&mut input);
		assert!(output.is_err());
		let mismatch = Mismatch::new("at most 0 occurrences", "1 occurrences");
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
	fn count_too_many_fails() {
		let parser = is('h');
		let mut input = Input::new_from_chars("hhhhello".chars(), None);
		let parser = count(&parser, 3);
		let output = parser(&mut input);
		assert!(output.is_err());
		let mismatch = Mismatch::new("at most 3 occurrences", "4 occurrences");
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 4),
				Some(mismatch)
			))
		);
	}

	#[test]
	fn count_all_same_succeeds() {
		let parser = is('h');
		let repeat_count: usize = 500;
		let tokens: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
		let mut input = Input::new_from_chars(tokens, None);
		let parser = count(&parser, repeat_count);
		let output = parser(&mut input).unwrap();
		assert_eq!(output, repeat_count); // The count matched the request.
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn count_one_different() {
		let parser = is('h');
		let repeat_count: usize = 500;
		let mut tokens: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
		tokens.push('x');
		let mut tail: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
		tokens.append(&mut tail);
		let mut input = Input::new_from_chars(tokens, None);
		let parser = count(&parser, repeat_count);
		let output = parser(&mut input).unwrap();
		assert_eq!(output, repeat_count);
		assert_eq!(input.consumed_count(), repeat_count); // Input was left intact.
		assert_eq!(any()(&mut input), Ok('x')); // Input was consumed as much as possible.
	}

	#[test]
	fn count_not_enough() {
		let parser = is('h');
		let mut tokens: Vec<_> = std::iter::repeat_n('h', 3).collect();
		tokens.push('x');
		tokens.push('y');
		let mut input = Input::new_from_chars(tokens, None);
		let parser = count(&parser, 4); // The 4th element is "x", so this should fail.
		let output = parser(&mut input);
		let mismatch = Mismatch::new('h', 'x');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 4),
				Some(mismatch)
			))
		);
	}

	#[test]
	fn count_not_enough_shortcut() {
		let parser = is('h').count(4);
		let mut tokens: Vec<_> = std::iter::repeat_n('h', 3).collect();
		tokens.push('x');
		tokens.push('y');
		let mut input = Input::new_from_chars(tokens, None);
		let output = parser(&mut input);
		let mismatch = Mismatch::new('h', 'x');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 4),
				Some(mismatch)
			))
		);
	}
}
