use crate::*;

#[test]
fn empty() {
	let any_parser = any();
	let end_comment_parser = is("*/");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_none() {
	let any_parser = any();
	let end_comment_parser = is("*/");
	let tokens: Vec<&str> = vec!["*/"];
	let mut input = Input::new(tokens);
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, Vec::<String>::new());
}

#[test]
fn success_multiple() {
	let any_parser = any();
	let end_comment_parser = is("*/");
	let tokens: Vec<&str> = vec!["hello", "world", "*/"];
	let mut input = Input::new(tokens);
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input).unwrap();
	assert_eq!(output, vec!["hello", "world"]);
}

#[test]
fn fail() {
	let any_parser = is("hello");
	let end_comment_parser = is("*/");
	let tokens: Vec<&str> = vec!["hello", "hello", "world"];
	let mut input = Input::new(tokens);
	let not_followed_parser = many_until(&any_parser, &end_comment_parser);
	let output = not_followed_parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert_eq!(any()(&mut input), Ok("world")); // Input was consumed while looking for the end.
}
