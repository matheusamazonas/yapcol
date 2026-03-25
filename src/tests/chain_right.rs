use crate::input::string::{new_string_input, CharToken};
use crate::*;

/// Implements a right-associative parser for subtraction operation and evaluates it.
fn parse_evaluate_right_subtraction() -> impl Parser<CharToken, i32> {
	|input| {
		let operand = satisfy(|c: &char| match c.to_digit(10) {
			Some(x) => Ok(x as i32),
			None => Err(Error::UnexpectedToken),
		});

		let operator = satisfy(|c: &char| match c {
			'-' => Ok(|a, b| a - b),
			_ => Err(Error::UnexpectedToken),
		});

		chain_right(&operand, &operator)(input)
	}
}

#[test]
fn empty() {
	let mut input = new_string_input("".chars());
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn one_operand() {
	let mut input = new_string_input("1".chars());
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(1));
}

#[test]
fn two_operands() {
	let mut input = new_string_input("9-7".chars());
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(2));
}

#[test]
fn tree_operands() {
	let mut input = new_string_input("3-1-2".chars()); // 3 - (1 - 2) = 4, not (3 - 1) - 2 = 0
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(4));
}
