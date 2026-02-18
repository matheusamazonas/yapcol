use crate::*;

#[test]
fn parse_maybe_success() {
	let parser = is(&(1));
	let mut tokens = vec![1];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(Some(1)));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_fail() {
	let parser = is(&(1));
	let mut tokens = vec![2];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_maybe_on_empty_input() {
	let parser = is(&(1));
	let mut tokens: Vec<i32> = vec![];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}
