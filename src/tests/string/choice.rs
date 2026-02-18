use crate::*;

#[test]
fn parse_choice_success() {
	let input1 = String::from("hello");
	let input2 = String::from("hallo");
	let input3 = String::from("hillo");
	let parser1 = is(&input1);
	let parser2 = is(&input2);
	let parser3 = is(&input3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut tokens = vec![input1.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input1);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let mut tokens = vec![input2.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input2);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let mut tokens = vec![input3.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input3);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let mut tokens = vec![String::from("hullo")];
	assert_eq!(parser_choice(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_fail() {
	let input1 = String::from("hello");
	let input2 = String::from("hallo");
	let input3 = String::from("hillo");
	let parser1 = is(&input1);
	let parser2 = is(&input2);
	let parser3 = is(&input3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut tokens = vec![String::from("hullo")];
	let output = parser_choice(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
