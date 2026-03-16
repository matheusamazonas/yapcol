use crate::*;

/// Implements a right-associative parser for subtraction operation and evaluates it.
fn parse_evaluate_right_subtraction<I>() -> impl Parser<I, i32>
where
	I: Iterator<Item = char>,
{
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
	let tokens: Vec<char> = Vec::new();
	let mut input = Input::new(tokens);
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn one_operand() {
	let tokens = vec!['1'];
	let mut input = Input::new(tokens);
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(1));
}

#[test]
fn two_operands() {
	let tokens: Vec<_> = "9-7".chars().collect();
	let mut input = Input::new(tokens);
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(2));
}

#[test]
fn tree_operands() {
	let tokens: Vec<_> = "3-1-2".chars().collect(); // 3 - (1 - 2) = 4, not (3 - 1) - 2 = 0 
	let mut input = Input::new(tokens);
	let output = parse_evaluate_right_subtraction()(&mut input);
	assert_eq!(output, Ok(4));
}
