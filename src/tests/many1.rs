use crate::*;

#[test]
fn parse_many1_empty() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let parser = is("hello");
	let tokens = vec!["hallo", "hillo", "hollo"];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let parser = is("hello");
	let tokens = vec!["hello", "hillo", "hollo"];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(input.consumed_count(), 1);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_multiple_matches() {
	let token_count = 100;
	let parser = is("hello");
	let tokens = std::iter::repeat_n("hello", token_count).collect::<Vec<_>>();
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| *x == "hello"));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_many1_partial_match_then_stop() {
	let parser = is("hello");
	let tokens = vec!["hello", "hello", "hillo", "hello"];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output, vec!["hello", "hello"]);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
