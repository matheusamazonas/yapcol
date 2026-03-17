use crate::*;

#[test]
fn success() {
	let parser = is("hello");
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(Some("hello")));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_non_consuming() {
	let parser = is("hello");
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_consuming() {
	let parser = |input: &mut Input<_>| match input.next_token() {
		Some(token) => {
			if token == "hello" {
				Ok(1)
			} else {
				Err(Error::UnexpectedToken)
			}
		}
		None => Err(Error::EndOfInput),
	};
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn empty() {
	let parser = is("hello");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
