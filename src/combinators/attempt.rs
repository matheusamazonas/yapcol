use crate::{InputToken, Parser};

/// Creates a parser that does not consume input in case the given parser fails.
///
/// # Outcome
///
/// The outcome of this combinator is based on the outcome of its parser argument.
///
/// # Input consumption
///
/// This combinator consumes input if the argument parser succeeds while consuming input.
/// Otherwise, it does not consume input.
///
/// # Look-ahead and backtracking
///
/// This combinator performs arbitrary lookahead, and will backtrack upon failure.
///
/// # Common usage
///
/// This combinator is often used alongside [`crate::either()`] whenever both input parsers share a
/// prefix. By doing so, we prevent [`crate::either()`] from failing if its first parser argument
/// failed while consuming input. For example:
/// ```rust,ignore
/// // Instead of this, where `either` would fail early and not even try applying `parser2`.
/// let parser = either(&parser1, &parser2);
/// // Do this, so if `parser1` fails consuming input, `parser2` will be applied.
/// let attempt_parser_1 = attempt(&parser1);
/// let parser = either(&attempt_parser_1, &parser2);
/// ```
///
/// # Shortcut
///
/// This combinator has a shortcut version: [`Parser::attempt`].
///
/// # Arguments
///
/// - `parser`: the parser to attempt.
///
/// # Examples
///
/// ```
/// use yapcol::input::Position;
/// use yapcol::{Error, Input, Mismatch, any, attempt, end_of_input, is};
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
/// let mismatch = Mismatch::new('1', '3');
/// assert_eq!(
/// 	output,
/// 	Err(Error::UnexpectedToken(
/// 		None,
/// 		Position::new(1, 2),
/// 		Some(mismatch)
/// 	))
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
		assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('h')))));
	}

	#[test]
	fn empty_shortcut() {
		let mut input = Input::new_from_chars("".chars(), None);
		let parser = is('h').attempt();
		let output = parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('h')))));
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
		let mismatch = Mismatch::new('h', 'j');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
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
		let mismatch = Mismatch::new('x', 'e');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
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
		let mismatch = Mismatch::new('h', 'e');
		assert_eq!(
			second,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
		// Input should still have "ello".
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn either_with_attempt_succeeds_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_h = is('h');
		let parser_e = is('e');
		let parser_attempt_h = attempt(&parser_h);
		let parser = either(&parser_attempt_h, &parser_e);
		let output = parser(&mut input);
		// Input was consumed because the first argument of `either` succeeded.
		assert_eq!(output, Ok('h'));
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn either_with_attempt_fails_not_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		let parser_e = is('e');
		let parser_l = is('l');
		let parser_attempt_e = attempt(&parser_e);
		let parser = either(&parser_attempt_e, &parser_l);
		let output = parser(&mut input);
		let mismatch = Mismatch::new('l', 'h');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 1),
				Some(mismatch)
			))
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
	fn either_with_attempt_on_consuming_parser_succeeds_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		// Create two parsers that share a prefix.
		let parser_h = is('h');
		let parser_e = is('e');
		let parser_l = is('l');
		let parser_hl = |input: &mut Input<_>| {
			let o1 = parser_h(input)?;
			let o2 = parser_l(input)?;
			Ok((o1, o2))
		};
		let parser_he = |input: &mut Input<_>| {
			let o1 = parser_h(input)?;
			let o2 = parser_e(input)?;
			Ok((o1, o2))
		};
		// Use `either` while the first uses `attempt`.
		let parser_attempt_hl = attempt(&parser_hl);
		let parser = either(&parser_attempt_hl, &parser_he);
		let output = parser(&mut input);
		// Even though the first parser failed consuming input, `either` succeeded because `attempt`
		// implements arbitrary lookahead and conserved input.
		assert_eq!(output, Ok(('h', 'e')));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}

	#[test]
	fn either_without_attempt_on_consuming_parser_fails_consuming() {
		let mut input = Input::new_from_chars("hello".chars(), None);
		// Create two parsers that share a prefix.
		let parser_h = is('h');
		let parser_e = is('e');
		let parser_l = is('l');
		let parser_hl = |input: &mut Input<_>| {
			let o1 = parser_h(input)?;
			let o2 = parser_l(input)?;
			Ok((o1, o2))
		};
		let parser_el = |input: &mut Input<_>| {
			let o1 = parser_e(input)?;
			let o2 = parser_l(input)?;
			Ok((o1, o2))
		};
		// Use `either` while the first does NOT use `attempt`.
		let parser = either(&parser_hl, &parser_el);
		let output = parser(&mut input);
		// The first parser failed while consuming input and `attempt` was not used, so the input
		// was consumed, and `either`'s second operand failed.
		let mismatch = Mismatch::new('l', 'e');
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(
				None,
				Position::new(1, 2),
				Some(mismatch)
			))
		);
		assert_eq!(any()(&mut input), Ok('e'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('l'));
		assert_eq!(any()(&mut input), Ok('o'));
		assert!(end_of_input()(&mut input).is_ok());
	}
}
