use crate::error::Error;
use crate::{satisfy, Parser};

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

fn parse_number<'a>() -> Parser<'a, String, Expression> {
	let f = |i: &String| match i.parse::<i32>() {
		Ok(number) => Ok(Expression::Number(number)),
		Err(_) => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

fn parse_operator<'a>() -> Parser<'a, String, Operator> {
	let f = |i: &String| match i.as_str() {
		"+" => Ok(Operator::Plus),
		"-" => Ok(Operator::Minus),
		_ => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

fn parse_operation<'a>() -> Parser<'a, String, Expression> {
	let f = |input: &mut Vec<String>| {
		let e1 = parse_number().parse(input)?;
		let op = parse_operator().parse(input)?;
		let e2 = parse_number().parse(input)?;
		let e1 = Box::new(e1);
		let e2 = Box::new(e2);
		Ok(Expression::Operation(e1, op, e2))
	};
	Parser::new(f)
}

#[test]
fn test_parse_number() {
	let number = "123";
	let mut input = vec![String::from(number)];
	let parser = parse_number();
	assert_eq!(parser.parse(&mut input), Ok(Expression::Number(123)));
}

#[test]
fn test_parse_addition() {
	let number1 = String::from("123");
	let number2 = String::from("456");
	let operation = String::from("+");
	let mut input = vec![number1, operation, number2];
	let parser = parse_operation();
	let output = parser.parse(&mut input);
	assert!(output.is_ok());
	let e = output.unwrap();
	match e {
		Expression::Operation(e1, op, e2) => {
			assert_eq!(op, Operator::Plus);
			assert_eq!(*e1, Expression::Number(123));
			assert_eq!(*e2, Expression::Number(456));
		},
		_ => assert!(false),
	}
}