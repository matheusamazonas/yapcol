use crate::input::string::new_string_input;
use crate::*;

#[test]
fn success_first() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = new_string_input("h".chars());
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, 'h');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_second() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = new_string_input("j".chars());
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, 'j');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_not_consuming() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = new_string_input("kello".chars());
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert_eq!(any()(&mut input), Ok('k')); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_consuming() {
	let parser1 = is('h');
	let parser2 = is('j');
	let mut input = new_string_input("hello".chars());
	let consuming_parser = |input: &mut Input<_>| {
		parser1(input)?;
		parser2(input)
	};
	let parse_option = option(&consuming_parser, &parser2);
	let output = parse_option(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert_eq!(any()(&mut input), Ok('e')); // Ensure that the input was consumed.
}
