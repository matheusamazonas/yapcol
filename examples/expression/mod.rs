#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Exponentiation,
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
