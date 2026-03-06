use crate::*;

#[test]
fn empty() {
	let tokens: Vec<i32> = Vec::new();
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_consumes() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Ok(1));
	// After the attempt succeeds, input should be consumed.
	assert_eq!(is(2)(&mut input), Ok(2));
	assert_eq!(is(3)(&mut input), Ok(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let tokens = vec![2, 3];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some(2));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_does_not_consume() {
	let tokens = vec![1, 3];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let consuming_parser = |input: &mut Input<_>| {
		let o1 = parser1(input)?; // Success, consumes 1.
		let o2 = parser2(input)?; // Fails on 3, consuming parser fails.
		Ok((o1, o2))
	};
	let output = attempt(&consuming_parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should be untouched even though the inner parser consumed.
	assert_eq!(input.next_token(), Some(1));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_twice() {
	let tokens = vec![1, 2];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let first = attempt(&parser)(&mut input);
	assert_eq!(first, Ok(1));
	// First attempt consumed the 1.
	let second = attempt(&parser)(&mut input);
	assert_eq!(second, Err(Error::UnexpectedToken));
	// Input should still have 2.
	assert_eq!(input.next_token(), Some(2));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_succeeds_consuming() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let parser_attempt_1 = attempt(&parser1);
	let parser = option(&parser_attempt_1, &parser2);
	let output = attempt(&parser)(&mut input);
	// Input was consumed because the first argument of `option` succeeded.
	assert_eq!(output, Ok(1));
	assert_eq!(input.next_token(), Some(2));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_fails_not_consuming() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let parser2 = is(2);
	let parser3 = is(3);
	let parser_attempt_1 = attempt(&parser2);
	let parser = option(&parser_attempt_1, &parser3);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// No input was consumed thanks to `attempt`.
	assert_eq!(input.next_token(), Some(1));
	assert_eq!(input.next_token(), Some(2));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_on_consuming_parser_succeeds_consuming() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is(1);
	let parser2 = is(2);
	let parser3 = is(3);
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
	let output = attempt(&parser)(&mut input);
	// Even though the first parser failed consuming input, `option` succeeded because `attempt`
	// implements arbitrary lookahead and conserved input.
	assert_eq!(output, Ok((1, 2)));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_without_option_on_consuming_parser_fails_not_consuming() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is(1);
	let parser2 = is(2);
	let parser3 = is(3);
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
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert_eq!(input.next_token(), Some(1));
	assert_eq!(input.next_token(), Some(2));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}
