use crate::*;

#[test]
fn parse_end_of_input_success() {
	let tokens: Vec<String> = vec![];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_end_of_input_fail() {
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_err());
}
