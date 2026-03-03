use crate::*;

#[test]
fn empty() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = Vec::new();
	let mut input = Input::new(tokens);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_consumes() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("foo");
	let parser1 = is(&token1);
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	let output = attempt(&parser1)(&mut input);
	assert_eq!(output, Ok(token1.clone()));
	// After attempt succeeds, input should be consumed.
	assert_eq!(is(&token2)(&mut input), Ok(token2.clone()));
	assert_eq!(is(&token3)(&mut input), Ok(token3.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let tokens = vec![token2.clone()];
	let mut input = Input::new(tokens);
	let parser1 = is(&token1);
	let output = attempt(&parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_does_not_consume() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone()];
	let mut input = Input::new(tokens);
	let consuming_parser = |input: &mut Input<_>| {
		let o1 = is(&token1)(input)?; // Success, consumes token1.
		let o2 = is(&token3)(input)?; // Fails on token2, consuming parser fails.
		Ok((o1, o2))
	};
	let output = attempt(&consuming_parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should be rewound even though the inner parser consumed.
	assert_eq!(input.next_token(), Some(token1.clone()));
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_twice() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let parser = is(&token1);
	let tokens = vec![token1.clone(), token2.clone()];
	let mut input = Input::new(tokens);
	let first = attempt(&parser)(&mut input);
	assert_eq!(first, Ok(token1.clone()));
	// First attempt consumed token1.
	let second = attempt(&parser)(&mut input);
	assert_eq!(second, Err(Error::UnexpectedToken));
	// Input should still have token2.
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_succeeds_consuming() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let parser_attempt_1 = attempt(&parser1);
	let parser = option(&parser_attempt_1, &parser2);
	let output = attempt(&parser)(&mut input);
	// 1 was consumed because the first argument of `option` succeeded.
	assert_eq!(output, Ok(token1.clone()));
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert_eq!(input.next_token(), Some(token3.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_fails_not_consuming() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	let parser2 = is(&token2);
	let parser3 = is(&token3);
	let parser_attempt_1 = attempt(&parser2);
	let parser = option(&parser_attempt_1, &parser3);
	let output = attempt(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// No input was consumed thanks to `attempt`.
	assert_eq!(input.next_token(), Some(token1.clone()));
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert_eq!(input.next_token(), Some(token3.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_with_option_on_consuming_parser_succeeds_consuming() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let parser3 = is(&token3);
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
	assert_eq!(output, Ok((token1.clone(), token2.clone())));
	assert_eq!(input.next_token(), Some(token3.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn attempt_without_option_on_consuming_parser_fails_not_consuming() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	// Create two parsers that share a prefix.
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let parser3 = is(&token3);
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
	assert_eq!(input.next_token(), Some(token1.clone()));
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert_eq!(input.next_token(), Some(token3.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}
