use crate::*;

#[test]
fn success() {
	let mut input = Input::new_from_chars("".chars(), None);
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn fail() {
	let mut input = Input::new_from_chars("hello".chars(), None);
	assert!(end_of_input()(&mut input).is_err());
}
