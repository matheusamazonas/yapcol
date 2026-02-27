use crate::*;

#[test]
fn parse_choice_success() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let token3 = String::from("hillo");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let parser3 = is(&token3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let tokens = vec![token1.clone()];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, token1);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let tokens = vec![token2.clone()];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, token2);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let tokens = vec![token3.clone()];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, token3);
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let tokens = vec![String::from("hullo")];
	let mut input = Input::new(tokens);
	assert_eq!(parser_choice(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_fail() {
	let token1 = String::from("hello");
	let token2 = String::from("hallo");
	let token3 = String::from("hillo");
	let parser1 = is(&token1);
	let parser2 = is(&token2);
	let parser3 = is(&token3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let tokens = vec![String::from("hullo")];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
