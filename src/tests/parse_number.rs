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

#[test]
fn parse_many0_empty() {
	let parser = is(&(1));
	let mut tokens = vec![];
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output, vec![]);
}

#[test]
fn parse_many0_no_match_not_empty() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat(43).take(token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), 0);
	assert_eq!(tokens.len(), token_count);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many0_match() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat(42).take(token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert_eq!(tokens.len(), 0);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_many1_empty() {
	let parser = is(&(1));
	let mut tokens = vec![];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let parser = is(&(1));
	let mut tokens = vec![2, 3, 4];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let parser = is(&(1));
	let mut tokens = vec![1, 3, 4];
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], 1);
	assert_eq!(tokens.len(), 2);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_multiple_matches() {
	let token_count = 100;
	let parser = is(&(42));
	let mut tokens = std::iter::repeat(42).take(token_count).collect::<Vec<_>>();
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|&x| x == 42));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}
