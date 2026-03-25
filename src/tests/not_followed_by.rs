use crate::input::string::new_string_input;
use crate::*;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = new_string_input("".chars());
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn followed() {
	let parser = is('h');
	let mut input = new_string_input("h".chars());
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn not_followed() {
	let parser = is('h');
	let mut input = new_string_input("jello".chars());
	let not_followed_parser = not_followed_by(&parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Ok(()));
}

#[test]
fn look_ahead_followed() {
	// Inspiration: https://github.com/haskell/parsec/issues/8
	let parser = is('h');
	let mut input = new_string_input("hello".chars());
	let lookahead_parser = look_ahead(&parser);
	// Just ensure that it succeeds to prove a point.
	let output = lookahead_parser(&mut input);
	assert_eq!(output, Ok('h'));
	// Actually test.
	let mut input = new_string_input("hello".chars());
	let not_followed_parser = not_followed_by(&lookahead_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
