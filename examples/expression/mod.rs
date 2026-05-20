use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Exponentiation,
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Operator::Addition => write!(f, "+"),
			Operator::Subtraction => write!(f, "-"),
			Operator::Multiplication => write!(f, "*"),
			Operator::Division => write!(f, "/"),
			Operator::Exponentiation => write!(f, "^"),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
	Number(i32),
	Operation(Box<Expression>, Operator, Box<Expression>),
}

pub fn evaluate(expression: Expression) -> i32 {
	match expression {
		Expression::Number(number) => number,
		Expression::Operation(o1, op, o2) => match op {
			Operator::Addition => evaluate(*o1) + evaluate(*o2),
			Operator::Subtraction => evaluate(*o1) - evaluate(*o2),
			Operator::Multiplication => evaluate(*o1) * evaluate(*o2),
			Operator::Division => evaluate(*o1) / evaluate(*o2),
			Operator::Exponentiation => evaluate(*o1).pow(evaluate(*o2) as u32),
		},
	}
}
