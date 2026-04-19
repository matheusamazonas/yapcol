use crate::*;

#[test]
fn empty() {
	let mut input = Input::new_from_chars("".chars(), None);
	let output = any()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let mut input = Input::new_from_chars("abc".chars(), None);
	let output = any()(&mut input);
	assert_eq!(output, Ok('a'));
	// Using it twice returns the second token.
	let output = any()(&mut input);
	assert_eq!(output, Ok('b'));
}
