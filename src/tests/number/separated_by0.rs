use crate::*;

#[test]
fn empty() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn single_no_separator_succeeds() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![1];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Ok(vec![1]));
}

#[test]
fn single_dangling_separator_fails() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![1, 2];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn two_with_separator_succeeds() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![1, 2, 1];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Ok(vec![1, 1]));
}

#[test]
fn two_wrong_last_element_fails() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn two_no_separator_succeeds() {
	let parser1 = is(1);
	let parser2 = is(2);
	let tokens = vec![1, 1];
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input).unwrap();
	assert_eq!(output, vec![1]);
}

#[test]
fn many_properly_separated_succeeds() {
	let parser1 = is(1);
	let parser2 = is(2);
	let repeat_count = 10;
	// Repeating it `2 * repeat_count - 1` ensures that there isn't a dangling separator.
	let tokens: Vec<i32> = [1, 2]
		.iter()
		.cycle()
		.take(2 * repeat_count - 1)
		.cloned()
		.collect();
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count);
}

#[test]
fn many_dangling_separator_fails() {
	let parser1 = is(1);
	let parser2 = is(2);
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let tokens: Vec<i32> = [1, 2]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.cloned()
		.collect();
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn many_wrong_last_element_fails() {
	let parser1 = is(1);
	let parser2 = is(2);
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let mut tokens: Vec<i32> = [1, 2]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.cloned()
		.collect();
	tokens.push(3);
	let mut input = Input::new(tokens);
	let parser_separated_by0 = separated_by0(&parser1, &parser2);
	let output = parser_separated_by0(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
