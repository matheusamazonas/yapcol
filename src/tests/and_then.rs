use crate::input::position::Position;
use crate::*;

#[test]
fn success_simple() {
	let double_parser = is('2').and_then(is);
	let mut input = Input::new_from_chars("22".chars(), None);
	assert_eq!(double_parser(&mut input), Ok('2'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_chained() {
	let triple_parser = is('2').and_then(is).and_then(is);
	let mut input = Input::new_from_chars("222".chars(), None);
	assert_eq!(triple_parser(&mut input), Ok('2'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_simple() {
	let double_parser = is('2').and_then(is);
	let mut input = Input::new_from_chars("23".chars(), None);
	assert_eq!(
		double_parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 2)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_chained() {
	let triple_parser = is('2').and_then(is).and_then(is);
	let mut input = Input::new_from_chars("223".chars(), None);
	assert_eq!(
		triple_parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 3)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
