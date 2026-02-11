use crate::*;

#[test]
fn parse_right() {
	let hello = String::from("hello");
	let parse = is(&hello);
	let mut tokens = vec![hello.clone()];
	assert_eq!(parse(&mut tokens), Ok(hello.clone()));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_wrong() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser = is(&hello);
	let mut tokens = vec![hallo.clone()];
	assert_eq!(parser(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_option_first() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![hello.clone()];
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut tokens).unwrap();
	assert_eq!(output, hello);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![hallo.clone()];
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut tokens).unwrap();
	assert_eq!(output, hallo);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let other = String::from("other");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![other.clone()];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

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