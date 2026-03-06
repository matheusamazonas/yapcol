use crate::*;

#[test]
fn empty() {
	let tokens: Vec<&str> = Vec::new();
	let mut input = Input::new(tokens);
	let parser = is("hello");
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_consumes() {
	let tokens = vec!["hello", "world", "foo"];
	let mut input = Input::new(tokens);
	let parser = is("hello");
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Ok("hello"));
	// After attempt succeeds, input should be consumed.
	assert_eq!(is("world")(&mut input), Ok("world"));
	assert_eq!(is("foo")(&mut input), Ok("foo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let parser = is("hello");
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some("hallo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_does_not_consume() {
	let tokens = vec!["hello", "world"];
	let mut input = Input::new(tokens);
	let consuming_parser = |input: &mut Input<_>| {
		let o1 = is("hello")(input)?; // Success, consumes "hello".
		let o2 = is("hillo")(input)?; // Fails on "world", consuming parser fails.
		Ok((o1, o2))
	};
	let output = attempt(&consuming_parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should be rewound even though the inner parser consumed.
	assert_eq!(input.next_token(), Some("hello"));
	assert_eq!(input.next_token(), Some("world"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_twice() {
	let tokens = vec!["hello", "world"];
	let mut input = Input::new(tokens);
	let parser = is("hello");
	let first = attempt(&parser)(&mut input);
	assert_eq!(first, Ok("hello"));
	// First attempt consumed "hello".
	let second = attempt(&parser)(&mut input);
	assert_eq!(second, Err(Error::UnexpectedToken));
	// Input should still have "world".
	assert_eq!(input.next_token(), Some("world"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_succeeds_consuming() {
	let tokens = vec!["hello", "world", "hillo"];
	let mut input = Input::new(tokens);
	let parser1 = is("hello");
	let parser2 = is("world");
	let parser_attempt = attempt(&parser1);
	let parser = option(&parser_attempt, &parser2);
	let output = attempt(&parser)(&mut input);
	// Input was consumed because the first argument of `option` succeeded.
	assert_eq!(output, Ok("hello"));
	assert_eq!(input.next_token(), Some("world"));
	assert_eq!(input.next_token(), Some("hillo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_fails_not_consuming() {
	let tokens = vec!["hello", "world", "hillo"];
	let mut input = Input::new(tokens);
	let parser2 = is("world");
	let parser3 = is("hillo");
	let parser_attempt_1 = attempt(&parser2);
	let parser = option(&parser_attempt_1, &parser3);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// No input was consumed thanks to `attempt`.
	assert_eq!(input.next_token(), Some("hello"));
	assert_eq!(input.next_token(), Some("world"));
	assert_eq!(input.next_token(), Some("hillo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_on_consuming_parser_succeeds_consuming() {
	let tokens = vec!["hello", "world", "hillo"];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is("hello");
	let parser2 = is("world");
	let parser3 = is("hillo");
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
	assert_eq!(output, Ok(("hello", "world")));
	assert_eq!(input.next_token(), Some("hillo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_without_option_on_consuming_parser_fails_not_consuming() {
	let tokens = vec!["hello", "world", "hillo"];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is("hello");
	let parser2 = is("world");
	let parser3 = is("hillo");
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
	assert_eq!(input.next_token(), Some("hello"));
	assert_eq!(input.next_token(), Some("world"));
	assert_eq!(input.next_token(), Some("hillo"));
	assert!(end_of_input()(&mut input).is_ok());
}