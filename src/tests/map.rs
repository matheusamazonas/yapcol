use crate::*;
use input::position::Position;

#[test]
fn empty() {
	let parser = is('2').map(|c: char| c.to_digit(10));
	let mut input = Input::new_from_chars("".chars(), None);
	assert_eq!(parser(&mut input), Err(Error::EndOfInput));
}

#[test]
fn success_simple() {
	let parser = is('2').map(|c: char| c.to_digit(10));
	let mut input = Input::new_from_chars("2".chars(), None);
	assert_eq!(parser(&mut input), Ok(Some(2)));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_chained() {
	let parser = is('2')
		.map(|c: char| c.to_digit(10))
		.map(|o| o.unwrap())
		.map(|x| x * 3);
	let mut input = Input::new_from_chars("2".chars(), None);
	assert_eq!(parser(&mut input), Ok(6));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_simple() {
	let parser = is('2').map(|c: char| c.to_digit(10));
	let mut input = Input::new_from_chars("3".chars(), None);
	assert_eq!(
		parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_chained() {
	let parser = is('5')
		.map(|c: char| c.to_digit(10))
		.map(|o| o.unwrap())
		.map(|x| x * 7);
	let mut input = Input::new_from_chars("3".chars(), None);
	assert_eq!(
		parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
