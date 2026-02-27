use crate::*;

#[test]
fn parse_right() {
	let token = String::from("hello");
	let parse = is(&token);
	let tokens = vec![token.clone()];
	let mut input = Input::new(tokens);
	assert_eq!(parse(&mut input), Ok(token.clone()));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_wrong() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let parser = is(&token1);
	let tokens = vec![token2.clone()];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
