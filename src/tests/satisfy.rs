use crate::*;
use input::position::Position;

#[test]
fn digits() {
	let parser = satisfy(|token: &char| {
		if token.is_ascii_digit() {
			Some(*token)
		} else {
			None
		}
	});
	// Digits.
	let mut input = Input::new_from_chars("1".chars());
	assert_eq!(parser(&mut input), Ok('1'));
	assert!(end_of_input()(&mut input).is_ok());
	// Words fails and does not consume.
	let mut input = Input::new_from_chars("hello".chars());
	assert_eq!(
		parser(&mut input),
		Err(Error::UnexpectedToken(Position::new(1, 1)))
	);
	assert_eq!(any()(&mut input), Ok('h'));
	assert_eq!(any()(&mut input), Ok('e'));
}
