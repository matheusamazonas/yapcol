use crate::*;

#[test]
fn empty() {
	let tokens: Vec<i32> = Vec::new();
	let mut input = Input::new(tokens);
	let open = is(1);
	let close = is(3);
	let parser = is(1);
	let output = between(&open, &parser, &close)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let tokens: Vec<i32> = vec![1, 2, 1];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let output = between(&parser1, &parser2, &parser1)(&mut input);
	assert_eq!(output, Ok(2));
	assert!(input.next_token().is_none());
}

#[test]
fn fail_repeated() {
	let tokens: Vec<i32> = vec![1, 2, 2, 1];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let output = between(&parser1, &parser2, &parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn fail_no_middle() {
	let tokens: Vec<i32> = vec![1, 1];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let output = between(&parser1, &parser2, &parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn fail_swap() {
	let tokens: Vec<i32> = vec![2, 1, 2];
	let mut input = Input::new(tokens);
	let parser1 = is(1);
	let parser2 = is(2);
	let output = between(&parser1, &parser2, &parser1)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
