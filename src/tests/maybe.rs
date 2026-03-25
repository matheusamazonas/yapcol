use crate::input::string::{new_string_input, CharToken};
use crate::*;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = new_string_input("".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success() {
	let parser = is('h');
	let mut input = new_string_input("h".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(Some('h')));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_non_consuming() {
	let parser = is('h');
	let mut input = new_string_input("j".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_consuming() {
	let parser = |input: &mut Input<CharToken>| match input.next_token() {
		Some(token) => {
			if token.token_owned() == 'h' {
				Ok(1)
			} else {
				Err(Error::UnexpectedToken)
			}
		}
		None => Err(Error::EndOfInput),
	};
	let mut input = new_string_input("j".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
