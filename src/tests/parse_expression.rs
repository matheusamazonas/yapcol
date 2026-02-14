use crate::error::Error;
use crate::{end_of_input, satisfy};

#[derive(Debug, PartialEq)]
enum Operator {
	Plus,
	Minus,
}

#[derive(Debug, PartialEq)]
enum Expression {
	Number(i32),
	Operation(Box<Expression>, Operator, Box<Expression>),
}

fn parse_number() -> impl Fn(&mut Vec<String>) -> Result<Expression, Error> {
	let f = |i: &String| match i.parse::<i32>() {
		Ok(number) => Ok(Expression::Number(number)),
		Err(_) => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

fn parse_operator() -> impl Fn(&mut Vec<String>) -> Result<Operator, Error> {
	let f = |i: &String| match i.as_str() {
		"+" => Ok(Operator::Plus),
		"-" => Ok(Operator::Minus),
		_ => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

fn parse_operation() -> impl Fn(&mut Vec<String>) -> Result<Expression, Error> {
	|input: &mut Vec<String>| {
		let e1 = parse_number()(input)?;
		let op = parse_operator()(input)?;
		let e2 = parse_number()(input)?;
		let e1 = Box::new(e1);
		let e2 = Box::new(e2);
		Ok(Expression::Operation(e1, op, e2))
	}
}

#[test]
fn test_parse_number() {
	let number = "123";
	let mut input = vec![String::from(number)];
	let parser = parse_number();
	assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
}

fn assert_operation(root: Expression, x1: i32, operator: Operator, x2: i32) {
	match root {
		Expression::Operation(e1, op, o2) => {
			assert_eq!(op, operator);
			assert_eq!(*e1, Expression::Number(x1));
			assert_eq!(*o2, Expression::Number(x2));
		}
		_ => assert!(false),
	}
}

#[test]
fn test_parse_addition() {
	let number1 = 123;
	let number2 = 456;
	let input1 = number1.to_string();
	let input2 = number2.to_string();
	let operation = String::from("+");
	let mut tokens = vec![input1, operation, input2];
	let parser = parse_operation();
	let output = parser(&mut tokens);
	assert!(output.is_ok());
	let e = output.unwrap();
	assert_operation(e, number1, Operator::Plus, number2);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn test_parse_subtraction() {
	let number1 = 123;
	let number2 = 456;
	let input1 = number1.to_string();
	let input2 = number2.to_string();
	let operation = String::from("-");
	let mut tokens = vec![input1, operation, input2];
	let parser = parse_operation();
	let output = parser(&mut tokens);
	assert!(output.is_ok());
	let e = output.unwrap();
	assert_operation(e, number1, Operator::Minus, number2);
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn test_parse_invalid_operation() {
	let number1 = 123;
	let number2 = 456;
	let input1 = number1.to_string();
	let input2 = number2.to_string();
	let operation = String::from("%");
	let mut tokens = vec![input1, operation, input2];
	let parser = parse_operation();
	let output = parser(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}
