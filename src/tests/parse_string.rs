use crate::*;

#[test]
fn parse_right() {
	let hello = String::from("hello");
	let parse = is(&hello);
	let mut tokens = vec![hello.clone()];
	assert_eq!(parse(&mut tokens), Ok(hello.clone()));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_wrong() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser = is(&hello);
	let mut tokens = vec![hallo.clone()];
	assert_eq!(parser(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_option_first() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![hello.clone()];
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut tokens).unwrap();
	assert_eq!(output, hello);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![hallo.clone()];
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut tokens).unwrap();
	assert_eq!(output, hallo);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let other = String::from("other");
	let parser1 = is(&hello);
	let parser2 = is(&hallo);
	let mut tokens = vec![other.clone()];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_maybe_success() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![hello.clone()];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(Some(hello.clone())));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_maybe_fail() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![String::from("hallo")];
	let parser_maybe = maybe(&parser);
	assert_eq!(parser_maybe(&mut tokens), Ok(None));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many0_empty() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![];
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), 0);
}

#[test]
fn parse_many0_no_match_not_empty() {
	let token_count = 100;
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = std::iter::repeat(String::from("hallo")).take(token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), 0);
	assert_eq!(tokens.len(), token_count);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many0_match() {
	let token_count = 100;
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = std::iter::repeat(hello.clone()).take(token_count).collect::<Vec<_>>();
	let parser_many0 = many0(&parser);
	let output = parser_many0(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert_eq!(tokens.len(), 0);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_many1_empty() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::EndOfInput));
}

#[test]
fn parse_many1_no_match() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![String::from("hallo"), String::from("hillo"), String::from("hollo")];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![String::from("hello"), String::from("hillo"), String::from("hollo")];
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), 1);
	assert_eq!(output[0], "hello");
	assert_eq!(tokens.len(), 2);
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_multiple_matches() {
	let token_count = 100;
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = std::iter::repeat(String::from("hello")).take(token_count).collect::<Vec<_>>();
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| x == "hello"));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}