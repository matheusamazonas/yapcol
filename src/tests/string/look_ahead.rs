use crate::*;

#[test]
fn empty() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = Vec::new();
	let mut input = Input::new(tokens);
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
}

#[test]
fn success_does_not_consume() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("foo");
	let parser1 = is(&token1);
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	let output = look_ahead(&parser1)(&mut input);
	assert_eq!(output, Ok(token1.clone()));
	// After look_ahead, input should still start with hello
	assert_eq!(is(&token1)(&mut input), Ok(token1.clone()));
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
	let output = look_ahead(&parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_consumes() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone()];
	let mut input = Input::new(tokens);
	let parser = |input: &mut Input<_>| {
		let output1 = is(&token1)(input)?; // Success, therefore it consumed.
		let output2 = is(&token3)(input)?; // Failed, so the whole parser fails consuming.
		Ok((output1, output2))
	};
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input was consumed.
	assert_eq!(input.next_token(), Some(token2.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_does_not_consume_on_failure() {
	let hello = String::from("hello");
	let world = String::from("world");
	let foo = String::from("foo");
	let parser = is(&hello);
	let tokens = vec![world.clone(), foo.clone()];
	let mut input = Input::new(tokens);
	let result = look_ahead(&parser)(&mut input);
	assert_eq!(result, Err(Error::UnexpectedToken));
	// Input should still be intact
	assert_eq!(is(&world)(&mut input), Ok(world.clone()));
	assert_eq!(is(&foo)(&mut input), Ok(foo.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_look_ahead_twice() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let tokens = vec![hello.clone()];
	let mut input = Input::new(tokens);
	let first = look_ahead(&parser)(&mut input);
	let second = look_ahead(&parser)(&mut input);
	assert_eq!(first, Ok(hello.clone()));
	assert_eq!(second, Ok(hello.clone()));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
