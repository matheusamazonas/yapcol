use crate::input::string::new_string_input;
use crate::*;

#[test]
fn success() {
	let parser = is('h');
	let mut input = new_string_input("h".chars());
	assert_eq!(parser(&mut input), Ok('h'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail() {
	let parser = is('j');
	let mut input = new_string_input("h".chars());
	assert_eq!(parser(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
