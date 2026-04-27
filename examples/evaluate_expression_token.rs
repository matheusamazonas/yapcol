use expression::{Expression, Operator, evaluate};
use std::fmt::Display;
use std::io;
use yapcol::input::Position;
use yapcol::{
	Error, Input, InputToken, Mismatch, Parser, attempt, between, chain_left, chain_right, is,
	option, satisfy,
};
mod expression;

#[derive(Debug, PartialEq, Clone)]
enum Token {
	Number(i32),
	Operator(Operator),
	OpenParenthesis,
	CloseParenthesis,
}

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Number(n) => write!(f, "{n}"),
			Token::Operator(op) => write!(f, "{op}"),
			Token::OpenParenthesis => write!(f, "("),
			Token::CloseParenthesis => write!(f, ")"),
		}
	}
}

#[derive(Debug, Clone)]
struct SourceToken {
	position: Position,
	token: Token,
}

impl InputToken for SourceToken {
	type Token = Token;

	fn token(&self) -> &Self::Token {
		&self.token
	}

	fn token_owned(self) -> Self::Token {
		self.token
	}

	fn position(&self) -> Position {
		self.position
	}
}

fn tokenize(input: String) -> Result<Vec<SourceToken>, Error> {
	let mut tokens = Vec::new();
	let input = input.chars().collect::<Vec<char>>();
	let mut i = 0;
	while i < input.len() {
		let token = match input[i] {
			'(' => Token::OpenParenthesis,
			')' => Token::CloseParenthesis,
			'+' => Token::Operator(Operator::Addition),
			'-' => Token::Operator(Operator::Subtraction),
			'*' => Token::Operator(Operator::Multiplication),
			'/' => Token::Operator(Operator::Division),
			'^' => Token::Operator(Operator::Exponentiation),
			c if c.is_numeric() => {
				let number: String = input
					.iter()
					.skip(i)
					.take_while(|&c| c.is_numeric())
					.collect();
				i += number.len() - 1;
				let number = number.parse().unwrap();
				Token::Number(number)
			}
			unsupported => {
				let position = Position::new(1, i + 1);
				let mismatch = Mismatch::without_expectation(unsupported);
				return Err(Error::UnexpectedToken(None, position, Some(mismatch)));
			}
		};
		let position = Position::new(1, i);
		i += 1;
		tokens.push(SourceToken { token, position });
	}

	Ok(tokens)
}

trait TokenExpressionParser: Parser<SourceToken, Expression> {}

impl<T> TokenExpressionParser for T where T: Fn(&mut Input<SourceToken>) -> Result<Expression, Error>
{}

fn parse_number() -> impl TokenExpressionParser {
	let f = |token: &Token| match token {
		Token::Number(number) => Some(Expression::Number(*number)),
		_ => None,
	};
	satisfy(f).with_expectation("number")
}

fn build_operation(op: Operator) -> impl Fn(Expression, Expression) -> Expression {
	move |o1, o2| Expression::Operation(Box::new(o1), op.clone(), Box::new(o2))
}

fn parse_operations(
	operator1: Operator,
	operator2: Operator,
) -> impl Parser<SourceToken, Box<dyn Fn(Expression, Expression) -> Expression>> {
	move |input| {
		let parse_op1 = is(Token::Operator(operator1.clone()));
		let parse_op2 = is(Token::Operator(operator2.clone()));
		let parse_attempt_op1 = attempt(&parse_op1);
		let operator = option(&parse_attempt_op1, &parse_op2)(input)?;
		match operator {
			Token::Operator(op) => Ok(Box::new(build_operation(op))),
			t => Err(Error::UnexpectedToken(
				input.source_name(),
				input.position(),
				Some(Mismatch::new(Box::new("operator"), Box::new(t))),
			)),
		}
	}
}

fn parse_expression() -> impl TokenExpressionParser {
	|input| {
		let parse_operator = parse_operations(Operator::Addition, Operator::Subtraction);
		chain_left(&parse_factor(), &parse_operator).with_expectation("expression")(input)
	}
}

fn parse_factor() -> impl TokenExpressionParser {
	|input| {
		let parse_operator = parse_operations(Operator::Multiplication, Operator::Division);
		chain_left(&parse_exponentiation(), &parse_operator).with_expectation("factor")(input)
	}
}

fn parse_exponentiation() -> impl TokenExpressionParser {
	|input| {
		let parse_operator = is(Token::Operator(Operator::Exponentiation))
			.map(|_| build_operation(Operator::Exponentiation));
		chain_right(&parse_bottom(), &parse_operator).with_expectation("exponentiation")(input)
	}
}

fn parse_bottom() -> impl TokenExpressionParser {
	|input| {
		let parse_number = parse_number();
		let parse_open = is(Token::OpenParenthesis);
		let parse_expression = parse_expression();
		let parse_close = is(Token::CloseParenthesis);
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
				input.retain(|c| c != '\n');
				match tokenize(input.clone()) {
					Ok(tokens) => {
						let mut input = Input::new_from_tokens(tokens, Some("stdin".to_string()));
						match parse_expression()(&mut input) {
							Ok(e) => println!("Success: {:?}", evaluate(e)),
							Err(e) => println!("Failed to parse expression: {e}"),
						}
					}
					Err(e) => println!("Failed to tokenize: {e}"),
				}
			}
			Err(_) => println!("Failed to read input."),
		}
	}
}

#[cfg(test)]
mod tokenize_tests {
	use super::*;

	fn assert_tokens(tokens: &[SourceToken], expected: &[Token]) {
		assert_eq!(tokens.len(), expected.len());
		assert!(
			tokens
				.iter()
				.zip(expected.iter())
				.all(|(t, e)| t.token == *e)
		);
	}

	#[test]
	fn addition() {
		let input = String::from("+");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Operator(Operator::Addition)]);
	}

	#[test]
	fn subtraction() {
		let input = String::from("-");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Operator(Operator::Subtraction)]);
	}

	#[test]
	fn multiplication() {
		let input = String::from("*");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Operator(Operator::Multiplication)]);
	}

	#[test]
	fn division() {
		let input = String::from("/");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Operator(Operator::Division)]);
	}

	#[test]
	fn number_single() {
		let input = String::from("1");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Number(1)]);
	}

	#[test]
	fn number_multiple() {
		let input = String::from("167253571");
		let tokens = tokenize(input).unwrap();
		assert_tokens(&tokens, &vec![Token::Number(167253571)]);
	}

	#[test]
	fn addition_operation() {
		let input = String::from("15+3");
		let tokens = tokenize(input).unwrap();
		assert_tokens(
			&tokens,
			&vec![
				Token::Number(15),
				Token::Operator(Operator::Addition),
				Token::Number(3),
			],
		);
	}
}

#[cfg(test)]
mod evaluation_tests {
	use super::*;
	use yapcol::end_of_input;

	fn parse_and_evaluate(input: &str) -> i32 {
		let tokens = tokenize(String::from(input)).unwrap();
		let mut input = Input::new_from_tokens(tokens, None);
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
