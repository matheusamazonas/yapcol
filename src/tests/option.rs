use crate::*;
use input::position::Position;

#[test]
fn success_first() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = Input::new_from_chars("h".chars(), None);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, 'h');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_second() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = Input::new_from_chars("j".chars(), None);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, 'j');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_not_consuming() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = Input::new_from_chars("kello".chars(), None);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(
		parse_option(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert_eq!(any()(&mut input), Ok('k')); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_consuming() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = Input::new_from_chars("hello".chars(), None);
	let consuming_parser = |input: &mut Input<_>| {
		parser1(input)?;
		parser2(input)
	};
	let parse_option = option(&consuming_parser, &parser2);
	let output = parse_option(&mut input);
	assert_eq!(
		output,
		Err(Error::UnexpectedToken(None, Position::new(1, 2)))
	);
	assert_eq!(any()(&mut input), Ok('e')); // Ensure that the input was consumed.
}
