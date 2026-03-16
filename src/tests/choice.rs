use crate::*;

#[test]
fn parse_choice_success() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let parser3 = is("hillo");
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, "hello");
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, "hallo");
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let tokens = vec!["hillo"];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input).unwrap();
	assert_eq!(output, "hillo");
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let tokens = vec!["hullo"];
	let mut input = Input::new(tokens);
	assert_eq!(parser_choice(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_fail() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let parser3 = is("hillo");
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let tokens = vec!["hullo"];
	let mut input = Input::new(tokens);
	let output = parser_choice(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
