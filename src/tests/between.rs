use crate::input::position::Position;
use crate::*;

#[test]
fn empty() {
	let mut input = Input::new_from_chars("".chars());
	let output = between(&is('('), &is('h'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let mut input = Input::new_from_chars("(x)".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Ok('x'));
	assert_eq!(any()(&mut input), Err(Error::EndOfInput));
}

#[test]
fn fail_repeated() {
	let mut input = Input::new_from_chars("(xx)".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 3))));
}

#[test]
fn fail_no_middle() {
	let mut input = Input::new_from_chars("()".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 2))));
}

#[test]
fn fail_swap() {
	let mut input = Input::new_from_chars(")xx(".chars());
	let output = between(&is('('), &is('x'), &is(')'))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
}
