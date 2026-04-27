use crate::{Error, Input, InputToken, Parser};

fn separated_tail<P, S, IT, O, SO>(
	parser: &P,
	separator: &S,
) -> impl Fn(&mut Input<IT>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<IT, O>,
	S: Parser<IT, SO>,
	IT: InputToken,
{
	move |input, mut output| {
		while separator(input).is_ok() {
			let next = parser(input)?;
			output.push(next);
		}
		Ok(output)
	}
}

/// Creates a parser that parses zero or more occurrences of `parser`, separated by `separator`.
///
/// # Arguments
///
/// - `parser`: The parser whose occurrences we're collecting.
/// - `separator`: The separator parser, whose content we're not interested in.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, separated_by0};
///
/// let parser1 = is('1');
/// let parser2 = is('2');
/// let mut input = Input::new_from_chars("121".chars(), None);
/// let parser_separated_by0 = separated_by0(&parser1, &parser2);
/// let output = parser_separated_by0(&mut input);
/// assert_eq!(output, Ok("11".chars().collect()));
/// ```
pub fn separated_by0<P, S, IT, O, OS>(parser: &P, separator: &S) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	S: Parser<IT, OS>,
	IT: InputToken,
{
	move |input| match parser(input) {
		Ok(token) => {
			let output = vec![token];
			separated_tail(&parser, &separator)(input, output)
		}
		Err(Error::EndOfInput(_)) => Ok(vec![]),
		Err(_) => Ok(vec![]),
	}
}

/// Creates a parser that parses one or more occurrences of `parser`, separated by `separator`.
///
/// # Arguments
///
/// - `parser`: The parser whose occurrences we're collecting.
/// - `separator`: The separator parser, whose content we're not interested in.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, separated_by1};
///
/// let parser1 = is('1');
/// let parser2 = is('2');
/// let mut input = Input::new_from_chars("121".chars(), None);
/// let parser_separated_by1 = separated_by1(&parser1, &parser2);
/// let output = parser_separated_by1(&mut input);
/// assert_eq!(output, Ok("11".chars().collect()));
/// ```
pub fn separated_by1<P, S, IT, O, SO>(parser: &P, separator: &S) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	S: Parser<IT, SO>,
	IT: InputToken,
{
	move |input| {
		let first = parser(input)?;
		let output = vec![first];
		separated_tail(&parser, &separator)(input, output)
	}
}

#[cfg(test)]
mod tests {
	mod separated_by0 {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Ok(vec![]));
		}

		#[test]
		fn single_no_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Ok(vec!['1']));
		}

		#[test]
		fn single_dangling_separator_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('1')))));
		}

		#[test]
		fn two_with_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Ok(vec!['1', '1']));
		}

		#[test]
		fn two_wrong_last_element_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,2".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			let mismatch = Mismatch::new('1', '2');
			assert_eq!(
				output,
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 3),
					Some(mismatch)
				))
			);
		}

		#[test]
		fn two_no_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("11".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
			assert_eq!(output, vec!['1']);
		}

		#[test]
		fn many_properly_separated_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
			assert_eq!(output.len(), 10);
		}

		#[test]
		fn many_dangling_separator_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1,".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('1')))));
		}

		#[test]
		fn many_wrong_last_element_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1,2".chars(), None);
			let output = separated_by0(&parse_item, &parse_separator)(&mut input);
			let mismatch = Mismatch::new('1', '2');
			assert_eq!(
				output,
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 21),
					Some(mismatch)
				))
			);
		}
	}

	mod separated_by1 {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('1')))));
		}

		#[test]
		fn single_no_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Ok(vec!['1']));
		}

		#[test]
		fn single_dangling_separator_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('1')))));
		}

		#[test]
		fn two_with_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Ok(vec!['1', '1']));
		}

		#[test]
		fn two_wrong_last_element_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,2".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			let mismatch = Mismatch::new('1', '2');
			assert_eq!(
				output,
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 3),
					Some(mismatch)
				))
			);
		}

		#[test]
		fn two_no_separator_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("11".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input).unwrap();
			assert_eq!(output, vec!['1']);
		}

		#[test]
		fn many_properly_separated_succeeds() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input).unwrap();
			assert_eq!(output.len(), 10);
		}

		#[test]
		fn many_dangling_separator_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1,".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			assert_eq!(output, Err(Error::EndOfInput(Some(Box::new('1')))));
		}

		#[test]
		fn many_wrong_last_element_fails() {
			let parse_item = is('1');
			let parse_separator = is(',');
			let mut input = Input::new_from_chars("1,1,1,1,1,1,1,1,1,1,2".chars(), None);
			let output = separated_by1(&parse_item, &parse_separator)(&mut input);
			let mismatch = Mismatch::new('1', '2');
			assert_eq!(
				output,
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 21),
					Some(mismatch)
				))
			);
		}
	}
}
