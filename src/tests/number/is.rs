use crate::*;

#[test]
fn parse_right() {
	let parser = is(&1);
	let tokens = vec![1];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Ok(1));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_wrong() {
	let parser = is(&1);
	let tokens = vec![2];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_empty() {
	let parser = is(&1);
	let tokens = vec![];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Err(Error::EndOfInput));
}

#[test]
fn parse_negative_number() {
	let parser = is(&(-1));
	let tokens = vec![-1];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Ok(-1));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_zero_number() {
	let parser = is(&0);
	let tokens = vec![0];
	let mut input = Input::new(tokens);
	assert_eq!(parser(&mut input), Ok(0));
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}
