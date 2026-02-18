use crate::*;

#[test]
fn parse_maybe_success() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![hello.clone()];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(Some(hello.clone())));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_fail() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![String::from("hallo")];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
