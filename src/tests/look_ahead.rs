use crate::*;

#[test]
fn empty() {
	let parser = is("hello");
	let tokens: Vec<&str> = Vec::new();
	let mut input = Input::new(tokens);
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
}

#[test]
fn success_does_not_consume() {
	let parser1 = is("hello");
	let tokens = vec!["hello", "world", "foo"];
	let mut input = Input::new(tokens);
	let output = look_ahead(&parser1)(&mut input);
	assert_eq!(output, Ok("hello"));
	// After look_ahead, input should still start with hello
	assert_eq!(is("hello")(&mut input), Ok("hello"));
	assert_eq!(is("world")(&mut input), Ok("world"));
	assert_eq!(is("foo")(&mut input), Ok("foo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let parser1 = is("hello");
	let output = look_ahead(&parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some("hallo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_consumes() {
	let tokens = vec!["hello", "hallo"];
	let mut input = Input::new(tokens);
	let parser = |input: &mut Input<_>| {
		let output1 = is("hello")(input)?; // Success, therefore it consumed.
		let output2 = is("hillo")(input)?; // Failed, so the whole parser fails consuming.
		Ok((output1, output2))
	};
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input was consumed.
	assert_eq!(input.next_token(), Some("hallo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_does_not_consume_on_failure() {
	let parser = is("hello");
	let tokens = vec!["world", "foo"];
	let mut input = Input::new(tokens);
	let result = look_ahead(&parser)(&mut input);
	assert_eq!(result, Err(Error::UnexpectedToken));
	// Input should still be intact
	assert_eq!(is("world")(&mut input), Ok("world"));
	assert_eq!(is("foo")(&mut input), Ok("foo"));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_look_ahead_twice() {
	let parser = is("hello");
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	let first = look_ahead(&parser)(&mut input);
	let second = look_ahead(&parser)(&mut input);
	assert_eq!(first, Ok("hello"));
	assert_eq!(second, Ok("hello"));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
