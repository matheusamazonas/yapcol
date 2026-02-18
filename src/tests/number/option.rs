use crate::*;

#[test]
fn parse_option_first() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![1];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Ok(1));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![2];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Ok(2));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![3];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
