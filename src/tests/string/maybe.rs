use crate::*;

#[test]
fn parse_maybe_success() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let tokens = vec![hello.clone()];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(Some(hello.clone())));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_fail_non_consuming() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let tokens = vec![String::from("hallo")];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_maybe_fail_consuming() {
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
	let tokens = vec![String::from("hallo")];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_on_empty_input() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let tokens: Vec<String> = vec![];
	let mut input = Input::new(tokens);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut input), Ok(None));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
