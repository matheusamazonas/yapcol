use std::io;
use yapcol_rs::error::Error;
use yapcol_rs::input::Input;
use yapcol_rs::{Parser, attempt, between, chain_left, is, many0, option, satisfy};

#[derive(Debug, PartialEq, Clone)]
enum Operator {
	Addition,
	Subtraction,
	Multiplication,
	Division,
}

#[derive(Debug, PartialEq, Clone)]
enum Expression {
	Number(i32),
	Operation(Box<Expression>, Operator, Box<Expression>),
}

trait ExpressionParser<I>: Parser<I, Expression>
where
	I: Iterator<Item = char>,
{
}

impl<I, T> ExpressionParser<I> for T
where
	I: Iterator<Item = char>,
	T: Fn(&mut Input<I>) -> Result<Expression, Error>,
{
}

fn parse_digit<I>() -> impl Parser<I, char>
where
	I: Iterator<Item = char>,
{
	let f = |c: &char| {
		if c.is_ascii_digit() {
			Ok(*c)
		} else {
			Err(Error::UnexpectedToken)
		}
	};
	satisfy(f)
}

fn parse_number<I>() -> impl ExpressionParser<I>
where
	I: Iterator<Item = char>,
{
	|input| {
		let parse_digit = parse_digit();
		let digits = many0(&parse_digit)(input)?;
		let digits: String = digits.iter().collect();
		match digits.parse::<i32>() {
			Ok(number) => Ok(Expression::Number(number)),
			Err(_) => Err(Error::UnexpectedToken),
		}
	}
}

fn build_operation(op: Operator) -> impl Fn(Expression, Expression) -> Expression {
	move |o1, o2| Expression::Operation(Box::new(o1), op.clone(), Box::new(o2))
}

fn parse_expression<I>() -> impl ExpressionParser<I>
where
	I: Iterator<Item = char>,
{
	move |input| {
		let parse_operand = |input: &mut Input<I>| {
			let parse_plus = is('+');
			let parse_minus = is('-');
			let attempt_parse_plus = attempt(&parse_plus);
			let operator = option(&attempt_parse_plus, &parse_minus)(input)?;
			match operator {
				'+' => Ok(build_operation(Operator::Addition)),
				'-' => Ok(build_operation(Operator::Subtraction)),
				_ => Err(Error::UnexpectedToken),
			}
		};
		let factor_parser = parse_term();
		chain_left(&factor_parser, &parse_operand)(input)
	}
}

fn parse_term<I>() -> impl ExpressionParser<I>
where
	I: Iterator<Item = char>,
{
	move |input| {
		let parse_operand = |input: &mut Input<I>| {
			let parse_multiplication = is('*');
			let parse_division = is('/');
			let attempt_parse_multiplication = attempt(&parse_multiplication);
			let operator = option(&attempt_parse_multiplication, &parse_division)(input)?;
			match operator {
				'*' => Ok(build_operation(Operator::Multiplication)),
				'/' => Ok(build_operation(Operator::Division)),
				_ => Err(Error::UnexpectedToken),
			}
		};
		let factor_parser = parse_factor();
		chain_left(&factor_parser, &parse_operand)(input)
	}
}

fn parse_factor<I>() -> impl ExpressionParser<I>
where
	I: Iterator<Item = char>,
{
	move |input| {
		let number = parse_number();
		let open = is('(');
		let expression = parse_expression();
		let close = is(')');
		let parse_parenthesis = between(&open, &expression, &close);
		let parse_parenthesis = attempt(&parse_parenthesis);
		option(&parse_parenthesis, &number)(input)
	}
}

fn evaluate(expression: Expression) -> i32 {
	match expression {
		Expression::Number(number) => number,
		Expression::Operation(o1, op, o2) => match op {
			Operator::Addition => evaluate(*o1) + evaluate(*o2),
			Operator::Subtraction => evaluate(*o1) - evaluate(*o2),
			Operator::Multiplication => evaluate(*o1) * evaluate(*o2),
			Operator::Division => evaluate(*o1) / evaluate(*o2),
		},
	}
}

fn main() {
	let stdin = io::stdin();
	let input = &mut String::new();

	loop {
		println!("Enter expression, or 'q' to quit:");
		input.clear();
		match stdin.read_line(input) {
			Ok(_) if input.len() == 2 && input.starts_with('q') => break,
			Ok(_) => {
				let mut input = Input::new(input.chars());
				match parse_expression()(&mut input) {
					Ok(e) => println!("Success: {:?}", evaluate(e)),
					Err(e) => println!("Failed to parse expression: {:?}", e),
				}
			}
			Err(_) => println!("Failed to read input."),
		}
	}
}

#[cfg(test)]
mod parsing_tests {
	use super::*;
	use yapcol_rs::end_of_input;

	fn build_operation(x: i32, operator: Operator, y: i32) -> Expression {
		let operand1 = Box::new(Expression::Number(x));
		let operand2 = Box::new(Expression::Number(y));
		Expression::Operation(operand1, operator, operand2)
	}

	#[test]
	fn number() {
		let number = "123";
		let tokens: Vec<_> = number.chars().collect();
		let mut input = Input::new(tokens);
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn addition() {
		let number1 = 123;
		let number2 = 456;
		let tokens = format!("{number1}+{number2}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		assert_eq!(
			output,
			build_operation(number1, Operator::Addition, number2)
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn subtraction() {
		let number1 = 123;
		let number2 = 456;
		let tokens = format!("{number1}-{number2}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		assert_eq!(
			output,
			build_operation(number1, Operator::Subtraction, number2)
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn multiplication() {
		let number1 = 123;
		let number2 = 456;
		let tokens = format!("{number1}*{number2}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		assert_eq!(
			output,
			build_operation(number1, Operator::Multiplication, number2)
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn division() {
		let number1 = 123;
		let number2 = 456;
		let tokens = format!("{number1}/{number2}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		assert_eq!(
			output,
			build_operation(number1, Operator::Division, number2)
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn two_addition() {
		let number1 = 123;
		let number2 = 456;
		let number3 = 789;
		let tokens = format!("{number1}+{number2}+{number3}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		let expression1 = build_operation(number1, Operator::Addition, number2);
		let expression2 = Expression::Operation(
			Box::new(expression1),
			Operator::Addition,
			Box::new(Expression::Number(number3)),
		);
		assert_eq!(output, expression2);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn addition_and_subtraction() {
		let number1 = 123;
		let number2 = 456;
		let number3 = 789;
		let tokens = format!("{number1}+{number2}-{number3}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		let expression1 = build_operation(number1, Operator::Addition, number2);
		let expression2 = Expression::Operation(
			Box::new(expression1),
			Operator::Subtraction,
			Box::new(Expression::Number(number3)),
		);
		assert_eq!(output, expression2);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn addition_and_multiplication() {
		let number1 = 123;
		let number2 = 456;
		let number3 = 789;
		let tokens = format!("{number1}+{number2}*{number3}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		let expression1 = build_operation(number2, Operator::Multiplication, number3);
		let expression2 = Expression::Operation(
			Box::new(Expression::Number(number1)),
			Operator::Addition,
			Box::new(expression1),
		);
		assert_eq!(output, expression2);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn parenthesis_number() {
		let number = "(123)";
		let tokens: Vec<_> = number.chars().collect();
		let mut input = Input::new(tokens);
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn double_parenthesis_number() {
		let number = "((123))";
		let tokens: Vec<_> = number.chars().collect();
		let mut input = Input::new(tokens);
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn parenthesis_changes_precedence() {
		let number1 = 123;
		let number2 = 456;
		let number3 = 789;
		let tokens = format!("({number1}+{number2})*{number3}");
		let mut input = Input::new(tokens.chars());
		let parser = parse_expression();
		let expression1 = Expression::Operation(
			Box::new(Expression::Number(number1)),
			Operator::Addition,
			Box::new(Expression::Number(number2)),
		);
		let expression2 = Expression::Operation(
			Box::new(expression1),
			Operator::Multiplication,
			Box::new(Expression::Number(number3)),
		);

		assert_eq!(parser(&mut input), Ok(expression2));
	}
}

#[cfg(test)]
mod evaluation_tests {
	use super::*;
	use yapcol_rs::end_of_input;

	fn parse_and_evaluate(input: &str) -> i32 {
		let mut input = Input::new(input.chars());
		let expression = parse_expression()(&mut input).unwrap();
		assert!(end_of_input()(&mut input).is_ok());
		evaluate(expression)
	}

	#[test]
	fn single_number() {
		assert_eq!(parse_and_evaluate("42"), 42);
	}

	#[test]
	fn addition() {
		assert_eq!(parse_and_evaluate("10+5"), 15);
	}

	#[test]
	fn subtraction() {
		assert_eq!(parse_and_evaluate("10-3"), 7);
	}

	#[test]
	fn multiplication() {
		assert_eq!(parse_and_evaluate("4*5"), 20);
	}

	#[test]
	fn division() {
		assert_eq!(parse_and_evaluate("20/4"), 5);
	}

	#[test]
	fn addition_and_multiplication_precedence() {
		// Multiplication has higher precedence: 2+3*4 = 2+(3*4) = 14
		assert_eq!(parse_and_evaluate("2+3*4"), 14);
	}

	#[test]
	fn subtraction_and_division_precedence() {
		// Division has higher precedence: 20-10/2 = 20-(10/2) = 15
		assert_eq!(parse_and_evaluate("20-10/2"), 15);
	}

	#[test]
	fn two_additions() {
		// Left-associative: 1+2+3 = (1+2)+3 = 6
		assert_eq!(parse_and_evaluate("1+2+3"), 6);
	}

	#[test]
	fn mixed_operations() {
		// 10+2*3-1 = 10+(2*3)-1 = 15
		assert_eq!(parse_and_evaluate("10+2*3-1"), 15);
	}
}
