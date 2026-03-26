use crate::input::string::new_string_input;
use crate::*;

#[test]
fn digits() {
	let parser = satisfy(|token: &char| {
		if token.is_ascii_digit() {
			Ok(*token)
		} else {
			Err(Error::UnexpectedToken(Position::new(1, 1)))
		}
	});
	// Digits.
	let mut input = new_string_input("1".chars());
	assert_eq!(parser(&mut input), Ok('1'));
	assert!(end_of_input()(&mut input).is_ok());
	// Words fails and does not consume.
	let mut input = new_string_input("hello".chars());
	assert_eq!(parser(&mut input), Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert_eq!(any()(&mut input), Ok('h'));
	assert_eq!(any()(&mut input), Ok('e'));
}
