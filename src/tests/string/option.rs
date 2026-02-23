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

#[test]
fn parse_option_consuming_fails() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let is_hallo = is(&hallo);
	let mut tokens = vec![hello.clone(), hallo.clone()];
	let consuming_parser = |input: &mut Vec<String>| {
		// Consume regardless of success.
		let next = input.next().unwrap(); // `next` consumes input.
		if next.len().is_multiple_of(2) {
			is_hallo(input)
		} else {
			Err(Error::UnexpectedToken)
		}
	};
	let parse_option = option(&consuming_parser, &is_hallo);
	let output = parse_option(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn parse_option_not_consuming_succeeds() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let is_hello = is(&hello);
	let is_hallo = is(&hallo);
	let mut tokens = vec![hello.clone(), hallo.clone()];
	let non_consuming_parser = |input: &mut Vec<String>| {
		let next = input.next_as_ref().unwrap(); // `next_as_ref` does not consumes input.
		if next.len().is_multiple_of(2) {
			input.next(); // Consume only if success.
			is_hallo(input)
		} else {
			Err(Error::UnexpectedToken)
		}
	};
	let parse_option = option(&non_consuming_parser, &is_hello);
	let output = parse_option(&mut tokens);
	assert_eq!(output, Ok(hello.clone()));
}