use crate::input::string::new_string_input;
use crate::*;

#[test]
fn success() {
	let mut input = new_string_input("".chars());
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn fail() {
	let mut input = new_string_input("hello".chars());
	assert!(end_of_input()(&mut input).is_err());
}
