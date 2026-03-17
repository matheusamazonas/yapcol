use crate::*;

#[test]
fn success() {
	let tokens: Vec<String> = vec![];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn fail() {
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_err());
}
