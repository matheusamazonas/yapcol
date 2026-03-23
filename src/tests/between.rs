use crate::*;

#[test]
fn empty() {
	let tokens: Vec<&str> = Vec::new();
	let mut input = Input::new(tokens);
	let output = between(&is("("), &is("hello"), &is(")"))(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let tokens = vec!["(", "hello", ")"];
	let mut input = Input::new(tokens);
	let output = between(&is("("), &is("hello"), &is(")"))(&mut input);
	assert_eq!(output, Ok("hello"));
	assert!(input.next_token().is_none());
}

#[test]
fn fail_repeated() {
	let tokens = vec!["(", "hello", "hello", ")"];
	let mut input = Input::new(tokens);
	let output = between(&is("("), &is("hello"), &is(")"))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn fail_no_middle() {
	let tokens = vec!["(", ")"];
	let mut input = Input::new(tokens);
	let output = between(&is("("), &is("hello"), &is(")"))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn fail_swap() {
	let tokens = vec![")", "hello", "("];
	let mut input = Input::new(tokens);
	let output = between(&is("("), &is("hello"), &is(")"))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
