use crate::*;

#[test]
fn parse_option_first() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let tokens = vec![token1.clone()];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, token1);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let tokens = vec![token2.clone()];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, token2);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let token3 = String::from("other");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let tokens = vec![token3.clone()];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_option_consuming_fails() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let tokens = vec![token1.clone(), token1.clone()];
	let mut input = Input::new(tokens);
	let consuming_parser = |input: &mut Input<_>| {
		parser1(input)?;
		parser2(input)
	};
	let parse_option = option(&consuming_parser, &parser2);
	let output = parse_option(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}