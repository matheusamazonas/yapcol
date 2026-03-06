use crate::*;

#[test]
fn empty() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens: Vec<&str> = vec![];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn single_no_separator_succeeds() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec!["hello"]));
}

#[test]
fn single_dangling_separator_fails() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens = vec!["hello", ","];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn two_with_separator_succeeds() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens = vec!["hello", ",", "hello"];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec!["hello", "hello"]));
}

#[test]
fn two_wrong_last_element_fails() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens = vec!["hello", ",", "world"];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn two_no_separator_succeeds() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let tokens = vec!["hello", "hello"];
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output, vec!["hello"]);
}

#[test]
fn many_properly_separated_succeeds() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let repeat_count = 10;
	// Repeating it `2 * repeat_count - 1` ensures that there isn't a dangling separator.
	let tokens: Vec<&str> = ["hello", ","]
		.iter()
		.copied()
		.cycle()
		.take(2 * repeat_count - 1)
		.collect();
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count);
}

#[test]
fn many_dangling_separator_fails() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let tokens: Vec<&str> = ["hello", ","]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.copied()
		.collect();
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn many_wrong_last_element_fails() {
	let parse_item = is("hello");
	let parse_separator = is(",");
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let mut tokens: Vec<&str> = ["hello", ","]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.copied()
		.collect();
	tokens.push("wrong");
	let mut input = Input::new(tokens);
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
