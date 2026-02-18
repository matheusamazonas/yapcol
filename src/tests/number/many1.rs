use crate::*;

#[test]
fn parse_many1_empty() {
	let parser = is(&(1));
	let mut tokens = vec![];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let parser = is(&(1));
	let mut tokens = vec![2, 3, 4];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let parser = is(&(1));
	let mut tokens = vec![1, 3, 4];
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], 1);
	assert_eq!(tokens.len(), 2);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_multiple_matches() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat_n(42, token_count).collect::<Vec<_>>();
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|&x| x == 42));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_many1_partial_match_then_stop() {
	let parser = is(&(42));
	let mut tokens = vec![42, 42, 7, 42];
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output, vec![42, 42]);
	assert_eq!(tokens, vec![7, 42]);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
