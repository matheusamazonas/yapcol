use crate::*;

#[test]
fn empty() {
	let open = String::from("(");
	let close = String::from(")");
	let middle = String::from("hello");
	let tokens: Vec<String> = Vec::new();
	let mut input = Input::new(tokens);
	let output = between(&is(&open), &is(&middle), &is(&close))(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success() {
	let open = String::from("(");
	let close = String::from(")");
	let middle = String::from("hello");
	let tokens = vec![open.clone(), middle.clone(), close.clone()];
	let mut input = Input::new(tokens);
	let output = between(&is(&open), &is(&middle), &is(&close))(&mut input);
	assert_eq!(output, Ok(middle.clone()));
	assert!(input.next_token().is_none());
}

#[test]
fn fail_repeated() {
	let open = String::from("(");
	let close = String::from(")");
	let middle = String::from("hello");
	let tokens = vec![open.clone(), middle.clone(), middle.clone(), close.clone()];
	let mut input = Input::new(tokens);
	let output = between(&is(&open), &is(&middle), &is(&close))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn fail_no_middle() {
	let open = String::from("(");
	let close = String::from(")");
	let middle = String::from("hello");
	let tokens = vec![open.clone(), close.clone()];
	let mut input = Input::new(tokens);
	let output = between(&is(&open), &is(&middle), &is(&close))(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}