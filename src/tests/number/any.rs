use crate::*;

#[test]
fn empty() {
	let tokens: Vec<i32> = Vec::new();
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Ok(1));
	// Using it twice returns the second token.
	let output = any()(&mut input);
	assert_eq!(output, Ok(2));
}
