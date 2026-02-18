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
fn parse_negative_number() {
	let parser = is(&(-1));
	let mut tokens = vec![-1];
	assert_eq!(parser(&mut tokens), Ok(-1));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_zero_number() {
	let parser = is(&0);
	let mut tokens = vec![0];
	assert_eq!(parser(&mut tokens), Ok(0));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}
