use crate::input::Position;
use crate::input::string::new_string_input;
use crate::*;

#[test]
fn empty() {
	let mut input = new_string_input("".chars());
	let output = between(&is('('), &is('h'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let mut input = new_string_input("(x)".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Ok('x'));
	assert!(input.next_token().is_none());
}

#[test]
fn fail_repeated() {
	let mut input = new_string_input("(xx)".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 3))));
}

#[test]
fn fail_no_middle() {
	let mut input = new_string_input("()".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 2))));
}

#[test]
fn fail_swap() {
	let mut input = new_string_input(")xx(".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
}
