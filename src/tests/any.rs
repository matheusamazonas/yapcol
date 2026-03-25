use crate::input::string::new_string_input;
use crate::*;

#[test]
fn empty() {
	let mut input = new_string_input("".chars());
	let output = any()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let mut input = new_string_input("abc".chars());
	let output = any()(&mut input);
	assert_eq!(output, Ok('a'));
	// Using it twice returns the second token.
	let output = any()(&mut input);
	assert_eq!(output, Ok('b'));
}
