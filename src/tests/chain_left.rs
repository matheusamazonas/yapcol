use crate::input::string::CharToken;
use crate::*;

/// Implements a left-associative parser for subtraction operation and evaluates it.
fn parse_evaluate_left_subtraction() -> impl Parser<CharToken, i32> {
	|input| {
		let operand = satisfy(|c: &char| c.to_digit(10).map(|x| x as i32));
		let operator = satisfy(|c: &char| match c {
			'-' => Some(|a, b| a - b),
			_ => None,
		});

		chain_left(&operand, &operator)(input)
	}
}

#[test]
fn empty() {
	let mut input = Input::new_from_chars("".chars(), None);
	let output = parse_evaluate_left_subtraction()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn one_operand() {
	let mut input = Input::new_from_chars("1".chars(), None);
	let output = parse_evaluate_left_subtraction()(&mut input);
	assert_eq!(output, Ok(1));
}

#[test]
fn two_operands() {
	let mut input = Input::new_from_chars("9-7".chars(), None);
	let output = parse_evaluate_left_subtraction()(&mut input);
	assert_eq!(output, Ok(2));
}

#[test]
fn tree_operands() {
	let mut input = Input::new_from_chars("3-1-2".chars(), None); // (3 - 1) - 2 = 0, not 3 - (1 - 2) = 4
	let output = parse_evaluate_left_subtraction()(&mut input);
	assert_eq!(output, Ok(0));
}
