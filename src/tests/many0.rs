use crate::*;

#[test]
fn parse_many0_empty() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), 0);
}

#[test]
fn parse_many0_no_match_not_empty() {
	let token_count = 100;
	let parser = is("hello");
	let tokens = std::iter::repeat_n("hallo", token_count).collect::<Vec<_>>();
	let mut input = Input::new(tokens);
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), 0);
	assert_eq!(input.consumed_count(), 0);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many0_match() {
	let token_count = 100;
	let parser = is("hello");
	let tokens = std::iter::repeat_n("hello", token_count).collect::<Vec<_>>();
	let mut input = Input::new(tokens);
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert_eq!(input.consumed_count(), token_count);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
