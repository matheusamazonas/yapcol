use crate::*;

#[test]
fn parse_end_of_input_success() {
	let mut tokens: Vec<i32> = vec![];
	assert!(end_of_input()(&mut tokens).is_ok());
}

#[test]
fn parse_end_of_input_fail() {
	let mut tokens = vec![1];
	assert!(end_of_input()(&mut tokens).is_err());
}
