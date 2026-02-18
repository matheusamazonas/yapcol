use crate::*;

#[test]
fn parse_many1_empty() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![
		String::from("hallo"),
		String::from("hillo"),
		String::from("hollo"),
	];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![
		String::from("hello"),
		String::from("hillo"),
		String::from("hollo"),
	];
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "hello");
	assert_eq!(tokens.len(), 2);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_multiple_matches() {
	let token_count = 100;
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = std::iter::repeat_n(String::from("hello"), token_count).collect::<Vec<_>>();
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| x == "hello"));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}
