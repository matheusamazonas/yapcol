use crate::*;

#[test]
fn parse_many1_empty() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = vec![];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = vec![
		String::from("hallo"),
		String::from("hillo"),
		String::from("hollo"),
	];
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = vec![
		String::from("hello"),
		String::from("hillo"),
		String::from("hollo"),
	];
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
	let token = String::from("hello");
	let parser = is(&token);
	let tokens = std::iter::repeat_n(String::from("hello"), token_count).collect::<Vec<_>>();
	let mut input = Input::new(tokens);
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| x == "hello"));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
