use crate::*;
use input::position::Position;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = Input::new_from_chars("".chars(), None);
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::EndOfInput));
}

#[test]
fn empty_shortcut() {
	let parser = is('h').many1();
	let mut input = Input::new_from_chars("".chars(), None);
	assert_eq!(parser(&mut input), Err(Error::EndOfInput));
}

#[test]
fn no_match() {
	let parser = is('h');
	let mut input = Input::new_from_chars("jklmno".chars(), None);
	let parser_many1 = many1(&parser);
	assert_eq!(
		parser_many1(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn no_match_shortcut() {
	let parser = is('h').many1();
	let mut input = Input::new_from_chars("jklmno".chars(), None);
	assert_eq!(
		parser(&mut input),
		Err(Error::UnexpectedToken(None, Position::new(1, 1)))
	);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn one_match() {
	let parser = is('h');
	let mut input = Input::new_from_chars("hallo".chars(), None);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(input.consumed_count(), 1);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn one_match_shortcut() {
	let parser = is('h').many1();
	let mut input = Input::new_from_chars("hallo".chars(), None);
	let output = parser(&mut input).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(input.consumed_count(), 1);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn multiple_matches() {
	let token_count = 100;
	let parser = is('h');
	let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
	let mut input = Input::new_from_chars(tokens, None);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| *x == 'h'));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn partial_match_then_stop() {
	let parser = is('h');
	let mut input = Input::new_from_chars("hhjklmnop".chars(), None);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output, vec!['h', 'h']);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
