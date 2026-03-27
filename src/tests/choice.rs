use crate::input::Position;
use crate::*;

#[test]
fn success() {
	let parser1 = is('h');
	let parser2 = is('e');
	let parser3 = is('l');
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut input = Input::new("h".chars());
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, 'h');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let mut input = Input::new("e".chars());
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, 'e');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let mut input = Input::new("l".chars());
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, 'l');
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let mut input = Input::new("u".chars());
	assert_eq!(
		parser_choice(&mut input),
		Err(Error::UnexpectedToken(Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail() {
	let parser1 = is('h');
	let parser2 = is('e');
	let parser3 = is('l');
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut input = Input::new("x".chars());
	let output = parser_choice(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
