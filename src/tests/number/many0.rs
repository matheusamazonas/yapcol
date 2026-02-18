use crate::*;

#[test]
fn parse_many0_empty() {
	let parser = is(&(1));
	let mut tokens = vec![];
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output, vec![]);
}

#[test]
fn parse_many0_no_match_not_empty() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat_n(43, token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), 0);
	assert_eq!(tokens.len(), token_count);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many0_match() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat_n(42, token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert_eq!(tokens.len(), 0);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}
