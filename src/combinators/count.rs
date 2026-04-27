use crate::{InputToken, Parser};

/// Creates a parser that applies the given parser exactly `count` times.
///
/// The parser succeeds only if the given parser succeeds exactly `count` times in a row,
/// returning a vector of the matched values. If the given parser fails at any point before
/// `count` applications, the whole parser fails.
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
/// assert_eq!(count(&parser, 3)(&mut input), Ok("111".chars().collect()));
/// assert_eq!(any()(&mut input), Ok('2')); // Remaining input after consuming 3 tokens.
///
/// // Fails when there are not enough matching tokens
/// let mut input = Input::new_from_chars("123".chars(), None);
/// assert!(count(&parser, 3)(&mut input).is_err());
///
/// // Succeeds with count = 0, returning an empty vector
/// let mut input = Input::new_from_chars("123".chars(), None);
/// assert_eq!(count(&parser, 0)(&mut input), Ok(vec![]));
///
/// // Fails on empty input when count > 0
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(count(&parser, 1)(&mut input).is_err());
/// ```
pub fn count<P, IT, O>(parser: &P, count: usize) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let mut output = Vec::with_capacity(count);
		for _ in 0..count {
			match parser(input) {
				Ok(token) => output.push(token),
				Err(e) => return Err(e),
			}
		}
		Ok(output)
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
		assert_eq!(output, Ok(vec![]));
	}

	#[test]
	fn count_0_not_empty() {
		let parser = is('h');
		let mut input = Input::new_from_chars("jello".chars(), None);
		let parser = count(&parser, 0);
		let output = parser(&mut input);
		assert_eq!(output, Ok(vec![]));
	}

	#[test]
	fn count_all_same() {
		let parser = is('h');
		let repeat_count: usize = 500;
		let tokens: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
		let mut input = Input::new_from_chars(tokens, None);
		let parser = count(&parser, repeat_count);
		let output = parser(&mut input).unwrap();
		assert_eq!(output.len(), repeat_count); // The count matched the request.
		assert!(output.iter().all(|x| *x == 'h')); // All values match the parser's.
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
		assert!(output.iter().all(|x| *x == 'h')); // All values match the parser's.
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
		let parser = count(&parser, 4); // The 4th element is "other", so this should fail.
		let output = parser(&mut input);
		let mismatch = Mismatch::with_expectation('h', 'x');
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
