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
fn parse_maybe_fail_non_consuming() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![String::from("hallo")];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_maybe_fail_consuming() {
	let parser = |input: &mut Vec<_>| match input.next() {
		Some(token) => if token == "hello" { Ok(1) } else { Err(Error::UnexpectedToken)},
		None => Err(Error::EndOfInput)
	};
	let mut tokens = vec![String::from("hallo")];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_on_empty_input() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens: Vec<String> = vec![];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}