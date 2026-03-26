use std::io;
use yapcol::error::Error;
use yapcol::input::Input;
use yapcol::{attempt, between, chain_left, chain_right, is, many0, option, satisfy, Parser};
mod expression;
use expression::{evaluate, Expression, Operator};
use yapcol::input::string::{new_string_input, CharToken};

trait StringExpressionParser: Parser<CharToken, Expression> {}

impl<T> StringExpressionParser for T where T: Fn(&mut Input<CharToken>) -> Result<Expression, Error> {}

fn parse_digit() -> impl Parser<CharToken, char> {
	let f = |c: &char| {
		if c.is_ascii_digit() { Some(*c) } else { None }
	};
	satisfy(f)
}

fn parse_number() -> impl StringExpressionParser {
	|input| {
		let parse_digit = parse_digit();
		let digits = many0(&parse_digit)(input)?;
		let digits: String = digits.iter().collect();
		match digits.parse::<i32>() {
			Ok(number) => Ok(Expression::Number(number)),
			Err(_) => Err(Error::UnexpectedToken(input.position())),
		}
	}
}

fn build_operation(op: Operator) -> impl Fn(Expression, Expression) -> Expression {
	move |o1, o2| Expression::Operation(Box::new(o1), op.clone(), Box::new(o2))
}

fn parse_expression() -> impl StringExpressionParser {
	|input| {
		let parse_operator = |input: &mut Input<_>| {
			let parse_plus = is('+');
			let parse_minus = is('-');
			let parse_attempt_plus = attempt(&parse_plus);
			let operator = option(&parse_attempt_plus, &parse_minus)(input)?;
			match operator {
				'+' => Ok(build_operation(Operator::Addition)),
				'-' => Ok(build_operation(Operator::Subtraction)),
				_ => Err(Error::UnexpectedToken(input.position())),
			}
		};
		chain_left(&parse_factor(), &parse_operator)(input)
	}
}

fn parse_factor() -> impl StringExpressionParser {
	|input| {
		let parse_operator = |input: &mut Input<_>| {
			let parse_multiplication = is('*');
			let parse_division = is('/');
			let parse_attempt_multiplication = attempt(&parse_multiplication);
			let operator = option(&parse_attempt_multiplication, &parse_division)(input)?;
			match operator {
				'*' => Ok(build_operation(Operator::Multiplication)),
				'/' => Ok(build_operation(Operator::Division)),
				_ => Err(Error::UnexpectedToken(input.position())),
			}
		};
		chain_left(&parse_exponentiation(), &parse_operator)(input)
	}
}

fn parse_exponentiation() -> impl StringExpressionParser {
	|input| {
		let parse_operator = |input: &mut Input<_>| match is('^')(input) {
			Ok(_) => Ok(build_operation(Operator::Exponentiation)),
			Err(_) => Err(Error::UnexpectedToken(input.position())),
		};
		chain_right(&parse_bottom(), &parse_operator)(input)
	}
}

fn parse_bottom() -> impl StringExpressionParser {
	|input| {
		let parse_number = parse_number();
		let parse_open = is('(');
		let parse_expression = parse_expression();
		let parse_close = is(')');
		let parse_parenthesis = between(&parse_open, &parse_expression, &parse_close);
		let parse_parenthesis = attempt(&parse_parenthesis);
		option(&parse_parenthesis, &parse_number)(input)
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
				let mut input = new_string_input(input.chars());
				match parse_expression()(&mut input) {
					Ok(e) => println!("Success: {:?}", evaluate(e)),
					Err(e) => println!("Failed to parse expression: {e}"),
				}
			}
			Err(_) => println!("Failed to read input."),
		}
	}
}

#[cfg(test)]
mod parsing_tests {
	use super::*;
	use yapcol::end_of_input;

	fn build_operation(x: i32, operator: Operator, y: i32) -> Expression {
		let operand1 = Box::new(Expression::Number(x));
		let operand2 = Box::new(Expression::Number(y));
		Expression::Operation(operand1, operator, operand2)
	}

	#[test]
	fn number() {
		let mut input = new_string_input("123".chars());
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn addition() {
		let number1 = 123;
		let number2 = 456;
		let tokens = format!("{number1}+{number2}");
		let mut input = new_string_input(tokens.chars());
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
		let mut input = new_string_input(tokens.chars());
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
		let mut input = new_string_input(tokens.chars());
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
		let mut input = new_string_input(tokens.chars());
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
		let mut input = new_string_input(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		// Addition is left-associative.
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
		let mut input = new_string_input(tokens.chars());
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
		let mut input = new_string_input(tokens.chars());
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
	fn simple_exponentiation() {
		let number1 = 12;
		let number2 = 3;
		let tokens = format!("{number1}^{number2}");
		let mut input = new_string_input(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		assert_eq!(
			output,
			build_operation(number1, Operator::Exponentiation, number2)
		);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn double_exponentiation() {
		let number1 = 2;
		let number2 = 3;
		let number3 = 4;
		let tokens = format!("{number1}^{number2}^{number3}");
		let mut input = new_string_input(tokens.chars());
		let parser = parse_expression();
		let output = parser(&mut input).unwrap();
		let expression1 = build_operation(number2, Operator::Exponentiation, number3);
		// Exponentiation is right-associative.
		let expression2 = Expression::Operation(
			Box::new(Expression::Number(number1)),
			Operator::Exponentiation,
			Box::new(expression1),
		);
		assert_eq!(output, expression2);
		assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
	}

	#[test]
	fn parenthesis_number() {
		let mut input = new_string_input("(123)".chars());
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn double_parenthesis_number() {
		let mut input = new_string_input("((123))".chars());
		let parser = parse_expression();
		assert_eq!(parser(&mut input), Ok(Expression::Number(123)));
	}

	#[test]
	fn parenthesis_changes_precedence() {
		let number1 = 123;
		let number2 = 456;
		let number3 = 789;
		let tokens = format!("({number1}+{number2})*{number3}");
		let mut input = new_string_input(tokens.chars());
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
	use yapcol::end_of_input;

	fn parse_and_evaluate(tokens: &str) -> i32 {
		let mut input = new_string_input(tokens.chars());
		let output = parse_expression()(&mut input);
		assert!(end_of_input()(&mut input).is_ok());
		evaluate(output.unwrap())
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
	fn simple_exponentiation() {
		assert_eq!(parse_and_evaluate("2^3"), 8);
	}

	#[test]
	fn double_exponentiation() {
		// Right-associative, equals to 4^(2^3) = 4^8 = 65536
		assert_eq!(parse_and_evaluate("4^2^3"), 65_536);
	}

	#[test]
	fn mixed_operations_no_exponentiation() {
		// 10+2*3-1 = 10+(2*3)-1 = 15
		assert_eq!(parse_and_evaluate("10+2*3-1"), 15);
	}

	#[test]
	fn mixed_operations_exponentiation() {
		// 28/4+2*3-2^3 = (28/4)+(2*3)-((2^3)) = 5
		assert_eq!(parse_and_evaluate("28/4+2*3-2^3"), 5);
	}

	#[test]
	fn mixed_operations_exponentiation_parenthesis() {
		// 28/4+2*(3-2)^3 = (28/4)+(2*((3-2)^3) = (28/4)+(2*(1^3)) = (28/4)+(2*1) = 7+2 = 9
		assert_eq!(parse_and_evaluate("28/4+2*(3-2)^3"), 9);
	}
}
