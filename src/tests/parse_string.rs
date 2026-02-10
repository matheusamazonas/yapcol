use crate::*;

#[test]
fn parse_string_right() {
	let hello = String::from("hello");
	let parse = is(&hello);
	let mut tokens = vec![hello.clone()];
	assert_eq!(parse(&mut tokens), Ok(hello.clone()));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_string_wrong() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser = is(&hello);
	let mut tokens = vec![hallo.clone()];
	assert_eq!(parser(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}