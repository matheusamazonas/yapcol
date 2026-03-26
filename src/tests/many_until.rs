use crate::input::string::new_string_input;
use crate::*;
use crate::input::Position;

#[test]
fn empty() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = new_string_input("".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_none() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = new_string_input("#".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, Vec::<char>::new());
}

#[test]
fn success_multiple() {
	let any_parser = any();
	let end_comment_parser = is('#');
	let mut input = new_string_input("Hello world #".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, "Hello world ".chars().collect::<Vec<_>>());
}

#[test]
fn fail() {
	let any_parser = is('x');
	let end_comment_parser = is('#');
	let mut input = new_string_input("xxxxxy".chars());
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 1))));
	assert_eq!(any()(&mut input), Ok('y')); // Input was consumed while looking for the end.
}
