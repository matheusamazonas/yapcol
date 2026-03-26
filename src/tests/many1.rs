use crate::input::string::new_string_input;
use crate::*;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = new_string_input("".chars());
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::EndOfInput));
}

#[test]
fn no_match() {
	let parser = is('h');
	let mut input = new_string_input("jklmno".chars());
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn one_match() {
	let parser = is('h');
	let mut input = new_string_input("hallo".chars());
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(input.consumed_count(), 1);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn multiple_matches() {
	let token_count = 100;
	let parser = is('h');
	let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
	let mut input = new_string_input(tokens);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| *x == 'h'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn partial_match_then_stop() {
	let parser = is('h');
	let mut input = new_string_input("hhjklmnop".chars());
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output, vec!['h', 'h']);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
