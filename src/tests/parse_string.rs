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
	let mut tokens = std::iter::repeat_n(String::from("hallo"), token_count)
		.collect::<Vec<_>>();
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
	let mut tokens = std::iter::repeat_n(hello.clone(), token_count).collect::<Vec<_>>();
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
	let mut tokens = vec![
		String::from("hallo"),
		String::from("hillo"),
		String::from("hollo"),
	];
	let parser_many1 = many1(&parser);
	assert_eq!(parser_many1(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_many1_one_match() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut tokens = vec![
		String::from("hello"),
		String::from("hillo"),
		String::from("hollo"),
	];
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
	let mut tokens = std::iter::repeat_n(String::from("hello"), token_count).collect::<Vec<_>>();
	let parser_many1 = many1(&parser);
	let output = parser_many1(&mut tokens).unwrap();
	assert_eq!(output.len(), token_count);
	assert!(output.iter().all(|x| x == "hello"));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_choice_success() {
	let input1 = String::from("hello");
	let input2 = String::from("hallo");
	let input3 = String::from("hillo");
	let parser1 = is(&input1);
	let parser2 = is(&input2);
	let parser3 = is(&input3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut tokens = vec![input1.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input1);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 2, success.
	let mut tokens = vec![input2.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input2);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 3, success.
	let mut tokens = vec![input3.clone()];
	let output = parser_choice(&mut tokens).unwrap();
	assert_eq!(output, input3);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
	// 4, fail.
	let mut tokens = vec![String::from("hullo")];
	assert_eq!(parser_choice(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_choice_fail() {
	let input1 = String::from("hello");
	let input2 = String::from("hallo");
	let input3 = String::from("hillo");
	let parser1 = is(&input1);
	let parser2 = is(&input2);
	let parser3 = is(&input3);
	let parsers: Vec<Box<dyn Parser<_, _>>> =
		vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)];
	let parser_choice = choice(&parsers);
	// 1, success.
	let mut tokens = vec![String::from("hullo")];
	let output = parser_choice(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_count_0_empty() {
	let input = String::from("hello");
	let parser = is(&input);
	let mut tokens: Vec<String> = vec![];
	let parser = count(&parser, 0);
	let output = parser(&mut tokens);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_0_not_empty() {
	let input = String::from("hello");
	let parser = is(&input);
	let mut tokens: Vec<String> = vec![String::from("other")];
	let parser = count(&parser, 0);
	let output = parser(&mut tokens);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_all_same() {
	let input = String::from("hello");
	let parser = is(&input);
	let repeat_count: u32 = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n(input.clone(), repeat_count as usize).collect();
	let parser = count(&parser, repeat_count);
	let output = parser(&mut tokens).unwrap();
	assert_eq!(output.len() as u32, repeat_count); // The count matched the request.
	assert!(output.iter().all(|x| *x == input));   // All values match the parser's.
	assert!(end_of_input()(&mut tokens).is_ok());  // Ensure that the input was consumed.
}

#[test]
fn parse_count_one_different() {
	let input = String::from("hello");
	let parser = is(&input);
	let repeat_count: u32 = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n(input.clone(), repeat_count as usize).collect();
	let other = String::from("other");
	tokens.push(other.clone());
	let mut tail = std::iter::repeat_n(input.clone(), repeat_count as usize).collect();
	tokens.append(&mut tail);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut tokens).unwrap();
	assert!(output.iter().all(|x| *x == input));     // All values match the parser's.
	assert_eq!(tokens.remove(0), other);             // Input was consumed as much as possible.
	assert_eq!(tokens.len(), repeat_count as usize); // Input was left intact.
}