use crate::{InputToken, Parser};

/// Parses at least one occurrence of `operand_parser`, separated by `operator_parser`. It combines
/// all values parsed by `operand_parser` into a final one using functions returned by
/// `operator_parser`, in a left-associative manner.
///
/// # Arguments
///
/// - `operand_parser`: Parses operands that will be combined into a final value, in a
///   left-associative manner.
/// - `operator_parser`: Operator's parser, which consumes input and returns a function that
///   combines output values into one.
///
/// # Examples
///
/// ```
/// // Implements evaluation of the subtraction ('-') operator as left-associative.
/// use yapcol::{satisfy, chain_left};
/// use yapcol::{Error, Input};
///
/// let operand = satisfy(|c: &char| c.to_digit(10));
///
/// let operator = satisfy(|c: &char| match c {
///     '-' => Some(|a, b| a - b),
///     _ => None,
/// });
///
/// let tokens: Vec<_> = "3-1-1".chars().collect();
/// let mut input = Input::new_from_chars(tokens, None);
/// let parser = chain_left(&operand, &operator);
/// let output = parser(&mut input);
/// assert_eq!(output, Ok(1)); // (3 - 1) - 1 = 1, not 3 - (1 - 1) = 3
/// ```
pub fn chain_left<P, IT, O, OP, F>(operand_parser: &P, operator_parser: &OP) -> impl Parser<IT, O>
where
	P: Parser<IT, O>,
	IT: InputToken,
	OP: Parser<IT, F>,
	F: Fn(O, O) -> O,
	O: Clone,
{
	move |input| {
		let o1 = operand_parser(input)?;
		parse_chain_left_tail(o1, operand_parser, operator_parser)(input)
	}
}

fn parse_chain_left_tail<P, IT, O, OP, F>(
	o1: O,
	parser: &P,
	operator_parser: &OP,
) -> impl Parser<IT, O>
where
	P: Parser<IT, O>,
	IT: InputToken,
	OP: Parser<IT, F>,
	F: FnOnce(O, O) -> O,
	O: Clone,
{
	move |input| match operator_parser(input) {
		Ok(operator) => {
			let o2 = parser(input)?;
			let output = operator(o1.clone(), o2);
			parse_chain_left_tail(output, parser, operator_parser)(input)
		}
		Err(_) => Ok(o1.clone()),
	}
}

/// Parses at least one occurrence of `operand_parser`, separated by `operator_parser`. It combines
/// all values parsed by `operand_parser` into a final one using functions returned by
/// `operator_parser`, in a right-associative manner.
///
/// # Arguments
///
/// - `operand_parser`: Parses operands that will be combined into a final value, in a
///   right-associative manner.
/// - `operator_parser`: Operator's parser, which consumes input and returns a function that
///   combines output values into one.
///
/// # Examples
///
/// ```
/// // Implements evaluation of the subtraction ('-') operator as right-associative.
/// use yapcol::{satisfy, chain_right};
/// use yapcol::{Error, Input};
///
/// let operand = satisfy(|c: &char| c.to_digit(10));
///
/// let operator = satisfy(|c: &char| match c {
///     '-' => Some(|a, b| a - b),
///     _ => None,
/// });
///
/// let tokens: Vec<_> = "3-1-1".chars().collect();
/// let mut input = Input::new_from_chars(tokens, None);
/// let parser = chain_right(&operand, &operator);
/// let output = parser(&mut input);
/// assert_eq!(output, Ok(3)); // 3 - (1 - 1) = 3, not (3 - 1) - 1 = 1
/// ```
pub fn chain_right<P, IT, O, OP, F>(operand_parser: &P, operator_parser: &OP) -> impl Parser<IT, O>
where
	P: Parser<IT, O>,
	IT: InputToken,
	OP: Parser<IT, F>,
	F: Fn(O, O) -> O,
{
	move |input| {
		let o1 = operand_parser(input)?;
		match operator_parser(input) {
			Ok(operator) => {
				let o2 = chain_right(operand_parser, operator_parser)(input)?;
				let output = operator(o1, o2);
				Ok(output)
			}
			Err(_) => Ok(o1),
		}
	}
}

#[cfg(test)]
mod tests {
	mod chain_left {
		use crate::input::CharToken;
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
			// (3 - 1) - 2 = 0, not 3 - (1 - 2) = 4
			let mut input = Input::new_from_chars("3-1-2".chars(), None);
			let output = parse_evaluate_left_subtraction()(&mut input);
			assert_eq!(output, Ok(0));
		}
	}

	mod chain_right {
		use crate::input::CharToken;
		use crate::*;

		/// Implements a right-associative parser for subtraction operation and evaluates it.
		fn parse_evaluate_right_subtraction() -> impl Parser<CharToken, i32> {
			|input| {
				let operand = satisfy(|c: &char| c.to_digit(10).map(|x| x as i32));
				let operator = satisfy(|c: &char| match c {
					'-' => Some(|a, b| a - b),
					_ => None,
				});

				chain_right(&operand, &operator)(input)
			}
		}

		#[test]
		fn empty() {
			let mut input = Input::new_from_chars("".chars(), None);
			let output = parse_evaluate_right_subtraction()(&mut input);
			assert_eq!(output, Err(Error::EndOfInput));
		}

		#[test]
		fn one_operand() {
			let mut input = Input::new_from_chars("1".chars(), None);
			let output = parse_evaluate_right_subtraction()(&mut input);
			assert_eq!(output, Ok(1));
		}

		#[test]
		fn two_operands() {
			let mut input = Input::new_from_chars("9-7".chars(), None);
			let output = parse_evaluate_right_subtraction()(&mut input);
			assert_eq!(output, Ok(2));
		}

		#[test]
		fn tree_operands() {
			// 3 - (1 - 2) = 4, not (3 - 1) - 2 = 0
			let mut input = Input::new_from_chars("3-1-2".chars(), None);
			let output = parse_evaluate_right_subtraction()(&mut input);
			assert_eq!(output, Ok(4));
		}
	}
}
