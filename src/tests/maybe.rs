use crate::input::Position;
use crate::*;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = Input::new("".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success() {
	let parser = is('h');
	let mut input = Input::new("h".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(Some('h')));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_non_consuming() {
	let parser = is('h');
	let mut input = Input::new("j".chars());
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
				Err(Error::UnexpectedToken(Position::new(1, 1)))
			}
		}
		None => Err(Error::EndOfInput),
	};
	let mut input = Input::new("j".chars());
	let parser_maybe = maybe(&parser);
	assert_eq!(
		parser_maybe(&mut input),
		Err(Error::UnexpectedToken(Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
