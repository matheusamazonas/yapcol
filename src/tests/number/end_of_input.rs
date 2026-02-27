use crate::*;

#[test]
fn parse_end_of_input_success() {
	let tokens: Vec<i32> = vec![];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_end_of_input_fail() {
	let tokens = vec![1];
	let mut input = Input::new(tokens);
	assert!(end_of_input()(&mut input).is_err());
}
