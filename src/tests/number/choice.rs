use crate::*;

#[test]
fn parse_choice_success() {
	let parser1 = is(&(1));
	let parser2 = is(&(2));
	let parser3 = is(&(3));
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut tokens = vec![1];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, 1);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let mut tokens = vec![2];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, 2);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let mut tokens = vec![3];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, 3);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let mut tokens = vec![4];
	assert_eq!(parser_choice(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_with_negative_and_zero() {
	let p_neg1 = is(&(-1));
	let p_zero = is(&0);
	let p_pos1 = is(&(1));
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(p_neg1), Box::new(p_zero), Box::new(p_pos1)];
	let parser_choice = choice(&parsers);
	// -1
	let mut tokens = vec![-1];
	assert_eq!(parser_choice(&mut tokens), Ok(-1));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 0
	let mut tokens = vec![0];
	assert_eq!(parser_choice(&mut tokens), Ok(0));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 2 fails
	let mut tokens = vec![2];
	assert_eq!(parser_choice(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_fail() {
	let parser1 = is(&(1));
	let parser2 = is(&(2));
	let parser3 = is(&(3));
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	let mut tokens = vec![4];
	let output = parser_choice(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
