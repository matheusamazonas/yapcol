use crate::*;
use input::position::Position;

#[test]
fn success() {
	let parser = is('h');
	let mut input = Input::new_from_chars("h".chars(), None);
	assert_eq!(parser(&mut input), Ok('h'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail() {
	let parser = is('j');
	let mut input = Input::new_from_chars("h".chars(), None);
	assert_eq!(
		parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
