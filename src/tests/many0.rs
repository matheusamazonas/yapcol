use crate::*;

#[test]
fn empty() {
	let parser = is('h');
	let mut input = Input::new_from_chars("".chars());
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), 0);
}

#[test]
fn no_match_not_empty() {
	let token_count = 100;
	let parser = is('h');
	let tokens = std::iter::repeat_n('j', token_count).collect::<Vec<_>>();
	let mut input = Input::new_from_chars(tokens);
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), 0);
	assert_eq!(input.consumed_count(), 0);
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn match_not_empty() {
	let token_count = 100;
	let parser = is('h');
	let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
	let mut input = Input::new_from_chars(tokens);
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut input).unwrap();
	assert_eq!(output.len(), token_count);
	assert_eq!(input.consumed_count(), token_count);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
