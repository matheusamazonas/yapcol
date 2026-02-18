use crate::*;

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
