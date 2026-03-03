use crate::*;
use std::vec;

#[test]
fn parse_option_first() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let tokens = vec![1];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Ok(1));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let tokens = vec![2];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Ok(2));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let tokens = vec![3];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_option_consuming_fails() {
	let is_1 = is(&1);
	let is_2 = is(&2);
	let tokens = vec![1, 3];
	let mut input = Input::new(tokens);
	let consuming_parser = |input: &mut Input<_>| {
		is_1(input)?;
		is_2(input)
	};
	let parse_option = option(&consuming_parser, &is_1);
	let output = parse_option(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
