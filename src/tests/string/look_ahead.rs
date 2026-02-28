use crate::*;

#[test]
fn parse_empty() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = Vec::new();
	let mut input = Input::new(tokens);
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
}

#[test]
fn parse_wrong_does_not_consume_input() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let parser1 = is(&token1);
	let tokens = vec![token2.clone()];
	let mut input = Input::new(tokens);
	let parse_look_ahead = look_ahead(&parser1)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::UnexpectedToken));
	assert_eq!(input.next_token(), Some(token2.clone())); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_does_not_consume_on_success() {
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
fn parse_right_multiple_elements() {
	let hello = String::from("hello");
	let world = String::from("world");
	let parser = is(&hello);
	let tokens = vec![hello.clone(), world.clone()];
	let mut input = Input::new(tokens);
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Ok(hello.clone()));
	assert_eq!(input.next_token(), Some(hello.clone())); // Ensure the input was NOT consumed.
	assert_eq!(input.next_token(), Some(world)); // Ensure the input was NOT consumed.
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