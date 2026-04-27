use crate::{InputToken, Parser};

/// Applies parsers `open` and `close` around `parser`. Often used for parenthesis, brackets, etc.
///
/// # Arguments
///
/// - `open`: The parser that "opens" the between.
/// - `parser`: The parser that goes between `open` and `close`, whose content we're interested in.
/// - `close`: The parser that "closes" the between.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, between, is};
///
/// let mut input = Input::new_from_chars("121".chars(), None);
/// let parser1 = is('1');
/// let parser2 = is('2');
/// let output = between(&parser1, &parser2, &parser1)(&mut input);
/// assert_eq!(output, Ok('2'));
/// ```
pub fn between<PO, PC, P, IT, O, OO, OC>(open: &PO, parser: &P, close: &PC) -> impl Parser<IT, O>
where
	PO: Parser<IT, OO>,
	PC: Parser<IT, OC>,
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		open(input)?;
		let output = parser(input)?;
		close(input)?;
		Ok(output)
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let mut input = Input::new_from_chars("".chars(), None);
		let output = between(&is('('), &is('h'), &is(')'))(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('(')))));
	}

	#[test]
	fn success() {
		let mut input = Input::new_from_chars("(x)".chars(), None);
		let output = between(&is('('), &is('x'), &is(')'))(&mut input);
		assert_eq!(output, Ok('x'));
		assert_eq!(any()(&mut input), Err(Error::EndOfInput(None)));
	}

	#[test]
	fn fail_repeated() {
		let mut input = Input::new_from_chars("(xx)".chars(), None);
		let output = between(&is('('), &is('x'), &is(')'))(&mut input);
		let mismatch = Mismatch::with_expectation(')', 'x');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 3),
				Some(mismatch)
			))
		);
	}

	#[test]
	fn fail_no_middle() {
		let mut input = Input::new_from_chars("()".chars(), None);
		let output = between(&is('('), &is('x'), &is(')'))(&mut input);
		let mismatch = Mismatch::with_expectation('x', ')');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
	}

	#[test]
	fn fail_swap() {
		let mut input = Input::new_from_chars(")xx(".chars(), None);
		let output = between(&is('('), &is('x'), &is(')'))(&mut input);
		let mismatch = Mismatch::with_expectation('(', ')');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
		);
	}
}
