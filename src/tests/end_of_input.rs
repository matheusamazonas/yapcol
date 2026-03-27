use crate::*;

#[test]
fn success() {
	let mut input = Input::new("".chars());
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn fail() {
	let mut input = Input::new("hello".chars());
	assert!(end_of_input()(&mut input).is_err());
}
