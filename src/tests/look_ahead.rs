use crate::*;
use input::position::Position;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = Input::new("".chars());
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
}

#[test]
fn success_does_not_consume() {
	let parser = is('h');
	let mut input = Input::new("hello".chars());
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Ok('h'));
	// After look_ahead, input should still start with hello
	assert_eq!(is('h')(&mut input), Ok('h'));
	assert_eq!(is('e')(&mut input), Ok('e'));
	assert_eq!(is('l')(&mut input), Ok('l'));
	assert_eq!(is('l')(&mut input), Ok('l'));
	assert_eq!(is('o')(&mut input), Ok('o'));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let parser = is('h');
	let mut input = Input::new("j".chars());
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
	// Input should still be intact.
	assert_eq!(any()(&mut input), Ok('j'));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_consumes() {
	let mut input = Input::new("he".chars());
	let parser = |input: &mut Input<_>| {
		let output1 = is('h')(input)?; // Success, therefore it consumed.
		let output2 = is('a')(input)?; // Failed, so the whole parser fails consuming.
		Ok((output1, output2))
	};
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 2))));
	// Input was consumed.
	assert_eq!(any()(&mut input), Ok('e'));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_does_not_consume_on_failure() {
	let parser = is('h');
	let mut input = Input::new("jello".chars());
	let result = look_ahead(&parser)(&mut input);
	assert_eq!(result, Err(Error::UnexpectedToken(Position::new(1, 1))));
	// Input should still be intact
	assert_eq!(is('j')(&mut input), Ok('j'));
	assert_eq!(is('e')(&mut input), Ok('e'));
	assert_eq!(is('l')(&mut input), Ok('l'));
	assert_eq!(is('l')(&mut input), Ok('l'));
	assert_eq!(is('o')(&mut input), Ok('o'));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_look_ahead_twice() {
	let parser = is('h');
	let mut input = Input::new("h".chars());
	let first = look_ahead(&parser)(&mut input);
	let second = look_ahead(&parser)(&mut input);
	assert_eq!(first, Ok('h'));
	assert_eq!(second, Ok('h'));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
