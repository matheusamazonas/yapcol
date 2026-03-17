use crate::*;

#[test]
fn empty() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn followed() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec!["hello"];
	let mut input = Input::new(tokens);
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn not_followed() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec!["world"];
	let mut input = Input::new(tokens);
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Ok(()));
}

#[test]
fn look_ahead_followed() {
	// Inspiration: https://github.com/haskell/parsec/issues/8
	let parser = is("hello");
	let tokens: Vec<&str> = vec!["hello", "world"];
	let mut input = Input::new(tokens);
	let lookahead_parser = look_ahead(&parser);
	// Just ensure that it succeeds to prove a point.
	let output = lookahead_parser(&mut input);
	assert_eq!(output, Ok("hello"));
	// Actually test.
	let tokens: Vec<&str> = vec!["hello", "world"];
	let mut input = Input::new(tokens);
	let not_followed_parser = not_followed_by(&lookahead_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
