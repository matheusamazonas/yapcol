use crate::*;

#[test]
fn success() {
	let parse = is("hello");
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	assert_eq!(parse(&mut input), Ok("hello"));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail() {
	let parser = is("hello");
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
