use crate::*;

#[test]
fn parse_number_right() {
	let parser = is(&(1));
	let mut tokens = vec![1];
	assert_eq!(parser.parse(&mut tokens), Ok(1));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_wrong() {
	let parser = is(&(1));
	let mut tokens = vec![2];
	assert_eq!(parser.parse(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_number_empty() {
	let parser = is(&(1));
	let mut tokens = vec![];
	assert_eq!(parser.parse(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_number_or_first() {
	let parser1 = is(&(1));
	let parser2 = is(&(2));
	let mut tokens = vec![1];
	let parser_or = parser1.or(&parser2);
	assert_eq!(parser_or.parse(&mut tokens), Ok(1));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_or_second() {
	let parser1 = is(&(1));
	let parser2 = is(&(2));
	let mut tokens = vec![2];
	let parser_or = parser1.or(&parser2);
	assert_eq!(parser_or.parse(&mut tokens), Ok(2));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_or_none() {
	let parser1 = is(&(1));
	let parser2 = is(&(2));
	let mut tokens = vec![3];
	let parser_or = parser1.or(&parser2);
	assert_eq!(parser_or.parse(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_number_maybe_success() {
	let parser = is(&(1));
	let mut tokens = vec![1];
	let parser_maybe = parser.maybe();
	assert_eq!(parser_maybe.parse(&mut tokens), Ok(Some(1)));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_maybe_fail() {
	let parser = is(&(1));
	let mut tokens = vec![2];
	let parser_maybe = parser.maybe();
	assert_eq!(parser_maybe.parse(&mut tokens), Ok(None));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}
