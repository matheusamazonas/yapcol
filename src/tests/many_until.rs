use crate::*;
use input::position::Position;

#[test]
fn empty() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = Input::new_from_chars("".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_none() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = Input::new_from_chars("#".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, Vec::<char>::new());
}

#[test]
fn success_multiple() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = Input::new_from_chars("Hello world #".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, "Hello world ".chars().collect::<Vec<_>>());
}

#[test]
fn fail() {
	let any_parser = is('x');
	let end_comment_parser = is('#');
	let mut input = Input::new_from_chars("xxxxxy".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 6))));
	assert_eq!(any()(&mut input), Ok('y')); // Input was consumed while looking for the end.
}
