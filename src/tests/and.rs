use crate::input::position::Position;
use crate::*;

#[test]
fn empty() {
	let double_parser = is('2').and(is('3'));
	let mut input = Input::new_from_chars("".chars(), None);
	assert_eq!(double_parser(&mut input), Err(Error::EndOfInput));
}

#[test]
fn success_simple() {
	let double_parser = is('2').and(is('3'));
	let mut input = Input::new_from_chars("23".chars(), None);
	assert_eq!(double_parser(&mut input), Ok('3'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_chained() {
	let triple_parser = is('2').and(is('3')).and(is('4'));
	let mut input = Input::new_from_chars("234".chars(), None);
	assert_eq!(triple_parser(&mut input), Ok('4'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_simple() {
	let double_parser = is('2').and(is('3'));
	let mut input = Input::new_from_chars("22".chars(), None);
	assert_eq!(
		double_parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 2)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_chained() {
	let triple_parser = is('2').and(is('3')).and(is('4'));
	let mut input = Input::new_from_chars("233".chars(), None);
	assert_eq!(
		triple_parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 3)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
