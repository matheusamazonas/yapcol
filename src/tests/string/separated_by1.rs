use crate::*;

#[test]
fn empty() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens: Vec<String> = vec![];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn single_no_separator_succeeds() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens = vec![item.clone()];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec![item.clone()]));
}

#[test]
fn single_dangling_separator_fails() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens = vec![item.clone(), separator.clone()];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn two_with_separator_succeeds() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens = vec![item.clone(), separator.clone(), item.clone()];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec![item.clone(), item.clone()]));
}

#[test]
fn two_wrong_last_element_fails() {
	let item = String::from("hello");
	let separator = String::from(",");
	let other = String::from("world");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens = vec![item.clone(), separator.clone(), other.clone()];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn two_no_separator_succeeds() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let tokens = vec![item.clone(), item.clone()];
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output, vec![item.clone()]);
}

#[test]
fn many_properly_separated_succeeds() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let repeat_count = 10;
	// Repeating it `2 * repeat_count - 1` ensures that there isn't a dangling separator.
	let tokens: Vec<String> = [item.clone(), separator.clone()]
		.iter()
		.cycle()
		.take(2 * repeat_count - 1)
		.cloned()
		.collect();
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count);
}

#[test]
fn many_dangling_separator_fails() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let tokens: Vec<String> = [item.clone(), separator.clone()]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.cloned()
		.collect();
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn many_wrong_last_element_fails() {
	let item = String::from("hello");
	let separator = String::from(",");
	let parse_item = is(&item);
	let parse_separator = is(&separator);
	let repeat_count = 100;
	// Repeating it `2 * repeat_count` ensures that there *is* a dangling separator.
	let mut tokens: Vec<String> = [item.clone(), separator.clone()]
		.iter()
		.cycle()
		.take(2 * repeat_count)
		.cloned()
		.collect();
	tokens.push(String::from("wrong"));
	let mut input = Input::new(tokens);
	let output = separated_by1(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
