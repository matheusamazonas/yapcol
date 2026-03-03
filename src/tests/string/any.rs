use crate::*;

#[test]
fn empty() {
	let tokens: Vec<String> = Vec::new();
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let token1 = String::from("hello");
	let token2 = String::from("world");
	let token3 = String::from("hillo");
	let tokens = vec![token1.clone(), token2.clone(), token3.clone()];
	let mut input = Input::new(tokens);
	let output = any()(&mut input);
	assert_eq!(output, Ok(token1.clone()));
	// Using it twice returns the second token.
	let output = any()(&mut input);
	assert_eq!(output, Ok(token2.clone()));
}
