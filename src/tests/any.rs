use crate::*;

#[test]
fn empty() {
	let tokens: Vec<&str> = Vec::new();
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let tokens = vec!["hello", "world", "hillo"];
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Ok("hello"));
	// Using it twice returns the second token.
	let output = any()(&mut input);
	assert_eq!(output, Ok("world"));
}
