use crate::*;

#[test]
fn parse_number_right() {
	let parse1 = is(&1);
	let mut tokens = vec![1];
	assert_eq!(parse1(&mut tokens), Ok(1));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_wrong() {
	let parse1 = is(&1);
	let mut tokens = vec![2];
	assert_eq!(parse1(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_number_empty() {
	let parse1 = is(&1);
	let mut tokens = vec![];
	assert_eq!(parse1(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_number_or_first() {
	let parse1 = is(&1);
	let parse2 = is(&2);
	let mut tokens = vec![1];
	let parse_option = option(parse1, parse2);
	let parse_option = option(&parse1, &parse2);
	assert_eq!(parse_option(&mut tokens), Ok(1));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_or_second() {
	let parse1 = is(&1);
	let parse2 = is(&2);
	let mut tokens = vec![2];
	let parse_option = option(parse1, parse2);
	let parse_option = option(&parse1, &parse2);
	assert_eq!(parse_option(&mut tokens), Ok(2));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_or_none() {
	let parse1 = is(&1);
	let parse2 = is(&2);
	let mut tokens = vec![3];
	let parse_option = option(parse1, parse2);
	let parse_option = option(&parse1, &parse2);
	assert_eq!(parse_option(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_number_maybe_success() {
	let parser = is(&(1));
	let mut tokens = vec![1];
	let parser_maybe = maybe(parser);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(Some(1)));
	assert_eq!(tokens.len(), 0); // Ensure that the input was consumed.
}

#[test]
fn parse_number_maybe_fail() {
	let parser = is(&(1));
	let mut tokens = vec![2];
	let parser_maybe = maybe(parser);
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert_eq!(tokens.len(), 1); // Ensure that the input was NOT consumed.
}
