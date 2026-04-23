use crate::{InputToken, Parser};

/// Creates a parser that does not consume input in case the given parser fails.
///
/// If the given parser succeeds, the matched value is returned. If the given parser consumed input,
/// this parser also does.
/// If the given parser fails while consuming input, this parser also fails but does not consume
/// input.
///
/// This combinator is often used alongside [`crate::option()`] whenever both input parsers share a
/// prefix. By doing so, we prevent [`crate::option()`] from failing if its first parser argument
/// failed while consuming input. For example:
/// ```rust,ignore
/// // Instead of this, where `option` would fail early and not even try applying `parser2`.
/// let parser = option(&parser1, &parser2);
/// // Do this, so if `parser1` fails consuming input, `parser2` will be applied.
/// let attempt_parser_1 = attempt(&parser1);
/// let parser = option(&attempt_parser_1, &parser2);
/// ```
///
/// Warning: this combinator implements arbitrary lookahead.
///
/// # Arguments
///
/// - `parser`: The parser to attempt.
///
/// # Examples
///
/// ```
/// use yapcol::input::Position;
/// use yapcol::{Error, Input, any, attempt, end_of_input, is};
///
/// // Succeeds consuming input.
/// let mut input = Input::new_from_chars("123".chars(), None);
/// let parser1 = is('1');
/// assert_eq!(attempt(&parser1)(&mut input), Ok('1'));
/// assert_eq!(any()(&mut input), Ok('2')); // Input was consumed.
///
/// // Fails without consuming input.
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert!(attempt(&parser1)(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('2')); // Input was not consumed.
/// assert_eq!(any()(&mut input), Ok('3')); // Input was not consumed.
///
/// // Fails without consuming input if the parser consumes.
/// let mut input = Input::new_from_chars("13".chars(), None);
/// let consuming_parser = |input: &mut Input<_>| {
/// 	let o1 = parser1(input)?;
/// 	let o2 = parser1(input)?;
/// 	Ok((o1, o2))
/// };
/// let output = attempt(&consuming_parser)(&mut input);
/// assert_eq!(
/// 	output,
/// 	Err(Error::UnexpectedToken(None, Position::new(1, 2)))
/// );
/// assert_eq!(any()(&mut input), Ok('1')); // Input was not consumed.
///
/// // Fails on empty input
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(attempt(&parser1)(&mut input).is_err());
/// ```
pub fn attempt<P, IT, O>(parser: &P) -> impl Parser<IT, O>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, output.is_err());
		output
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let mut input = Input::new_from_chars("".chars(), None);
		let parser = is('h');
		let output = attempt(&parser)(&mut input);
		assert_eq!(output, Err(Error::EndOfInput));
	}

	#[test]
	fn empty_shortcut() {
		let mut input = Input::new_from_chars("".chars(), None);
		let parser = is('h').attempt();
		let output = parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput));
	}

	#[test]
	fn success_consumes() {
		let mut input = Input::new_from_chars("hel".chars(), None);
		let parser = is('h');
		let output = attempt(&parser)(&mut input);
		assert_eq!(output, Ok('h'));
		// After attempt succeeds, input should be consumed.
		assert_eq!(is('e')(&mut input), Ok('e'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn success_consumes_shortcut() {
		let mut input = Input::new_from_chars("hel".chars(), None);
		let parser = is('h').attempt();
		let output = parser(&mut input);
		assert_eq!(output, Ok('h'));
		// After attempt succeeds, input should be consumed.
		assert_eq!(is('e')(&mut input), Ok('e'));
		assert_eq!(is('l')(&mut input), Ok('l'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn non_consuming_fail_does_not_consume() {
		let mut input = Input::new_from_chars("jello".chars(), None);
		let parser = is('h');
		let output = attempt(&parser)(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		// Input should still be intact.
		assert_eq!(any()(&mut input), Ok('j'));
	}

	#[test]
	fn consuming_fail_does_not_consume() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let consuming_parser = |input: &mut Input<_>| {
			let o1 = is('h')(input)?; // Success, consumes 'h'.
			let o2 = is('x')(input)?; // Fails on  'x', consuming parser fails.
			Ok((o1, o2))
		};
		let output = attempt(&consuming_parser)(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 2)))
		);
		// Input should be rewound even though the inner parser consumed.
		assert_eq!(any()(&mut input), Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn attempt_twice() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser = is('h');
		let first = attempt(&parser)(&mut input);
		assert_eq!(first, Ok('h'));
		// First attempt consumed 'h'.
		let second = attempt(&parser)(&mut input);
		assert_eq!(
			second,
			Err(Error::UnexpectedToken(None, Position::new(1, 2)))
		);
		// Input should still have "ello".
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn attempt_with_option_succeeds_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser1 = is('h');
		let parser2 = is('e');
		let parser_attempt = attempt(&parser1);
		let parser = option(&parser_attempt, &parser2);
		let output = attempt(&parser)(&mut input);
		// Input was consumed because the first argument of `option` succeeded.
		assert_eq!(output, Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn attempt_with_option_fails_not_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser1 = is('e');
		let parser2 = is('l');
		let parser_attempt_1 = attempt(&parser1);
		let parser = option(&parser_attempt_1, &parser2);
		let output = attempt(&parser)(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 1)))
		);
		// No input was consumed thanks to `attempt`.
		assert_eq!(any()(&mut input), Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn attempt_with_option_on_consuming_parser_succeeds_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		// Create two parsers that share a prefix.
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parser13 = |input: &mut Input<_>| {
			let o1 = parser1(input)?;
			let o2 = parser3(input)?;
			Ok((o1, o2))
		};
		let parser12 = |input: &mut Input<_>| {
			let o1 = parser1(input)?;
			let o2 = parser2(input)?;
			Ok((o1, o2))
		};
		// Use `option` while the first uses `attempt`.
		let parser_attempt_1 = attempt(&parser13);
		let parser = option(&parser_attempt_1, &parser12);
		let output = parser(&mut input);
		// Even though the first parser failed consuming input, `option` succeeded because `attempt`
		// implements arbitrary lookahead and conserved input.
		assert_eq!(output, Ok(('h', 'e')));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn attempt_without_option_on_consuming_parser_fails_not_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		// Create two parsers that share a prefix.
		let parser1 = is('h');
		let parser2 = is('e');
		let parser3 = is('l');
		let parser13 = |input: &mut Input<_>| {
			let o1 = parser1(input)?;
			let o2 = parser3(input)?;
			Ok((o1, o2))
		};
		let parser12 = |input: &mut Input<_>| {
			let o1 = parser1(input)?;
			let o2 = parser2(input)?;
			Ok((o1, o2))
		};
		// Use `option` while the first does NOT use `attempt`.
		let parser = option(&parser13, &parser12);
		let output = attempt(&parser)(&mut input);
		// The first parser failed consuming input and `attempt` was not used, so the input was
		// consumed, and `option`'s second operand failed.
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 2)))
		);
		assert_eq!(any()(&mut input), Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}
}
