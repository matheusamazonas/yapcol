use crate::*;

#[test]
fn parse_right() {
	let parser = is(&1);
	let mut tokens = vec![1];
	assert_eq!(parser(&mut tokens), Ok(1));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_wrong() {
	let parser = is(&1);
	let mut tokens = vec![2];
	assert_eq!(parser(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_empty() {
	let parser = is(&1);
	let mut tokens = vec![];
	assert_eq!(parser(&mut tokens), Err(Error::EndOfInput));
}

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

#[test]
fn parse_maybe_success() {
	let parser = is(&(1));
	let mut tokens = vec![1];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(Some(1)));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_fail() {
	let parser = is(&(1));
	let mut tokens = vec![2];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

