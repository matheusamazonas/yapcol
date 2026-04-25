use crate::{Error, Input, InputToken, Parser};

/// Applies `parser` zero or more times, returning a vector of matches.
/// This parser never fails: if no matches are found, it returns an empty vector.
///
/// # Arguments
///
/// - `parser`: The parser to possibly be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many0};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok("11".chars().collect()));
///
/// // Returns an empty vector when no matches are found (never fails)
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
///
/// // Returns an empty vector on empty input (never fails)
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
/// ```
pub fn many0<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let output: Vec<O> = Vec::new();
		many(parser)(input, output)
	}
}

/// Applies `parser` one or more times, returning a vector of matches.
/// This parser fails if no matches are found.
///
/// # Arguments
///
/// - `parser`: The parser to be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, is, many1};
///
/// // Matches multiple elements
/// let parser = is('1');
/// let mut input = Input::new_from_chars("112".chars(), None);
/// assert_eq!(many1(&parser)(&mut input), Ok("11".chars().collect()));
///
/// // Fails when no matches are found
/// let mut input = Input::new_from_chars("23".chars(), None);
/// assert!(many1(&parser)(&mut input).is_err());
///
/// // Fails on empty input
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert!(many1(&parser)(&mut input).is_err());
/// ```
pub fn many1<P, IT, O>(parser: &P) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input| {
		let mut output: Vec<O> = Vec::new();
		match parser(input) {
			Ok(token) => {
				output.push(token);
				many(parser)(input, output)
			}
			Err(e) => Err(e),
		}
	}
}

fn many<P, IT, O>(parser: &P) -> impl Fn(&mut Input<IT>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	|input, mut output| {
		loop {
			match parser(input) {
				Ok(token) => {
					output.push(token);
					continue;
				}
				Err(_) => return Ok(output),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	mod many0 {
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many0();
			let mut input = Input::new_from_chars("".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
		}

		#[test]
		fn no_match_not_empty() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('j', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_not_empty_shortcut() {
			let token_count = 100;
			let parser = is('h').many0();
			let tokens = std::iter::repeat_n('j', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 0);
			assert_eq!(input.consumed_count(), 0);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn match_not_empty() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many0 = many0(&parser);
			let output = parser_many0(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert_eq!(input.consumed_count(), token_count);
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn match_not_empty_shortcut() {
			let token_count = 100;
			let parser = is('h').many0();
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert_eq!(input.consumed_count(), token_count);
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}
	}

	mod many1 {
		use crate::input::Position;
		use crate::*;

		#[test]
		fn empty() {
			let parser = is('h');
			let mut input = Input::new_from_chars("".chars(), None);
			let parser_many1 = many1(&parser);
			assert_eq!(parser_many1(&mut input), Err(Error::EndOfInput));
		}

		#[test]
		fn empty_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("".chars(), None);
			assert_eq!(parser(&mut input), Err(Error::EndOfInput));
		}

		#[test]
		fn no_match() {
			let parser = is('h');
			let mut input = Input::new_from_chars("jklmno".chars(), None);
			let parser_many1 = many1(&parser);
			let mismatch = Mismatch::new('h', 'j');
			assert_eq!(
				parser_many1(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn no_match_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("jklmno".chars(), None);
			let mismatch = Mismatch::new('h', 'j');
			assert_eq!(
				parser(&mut input),
				Err(Error::UnexpectedToken(
					None,
					Position::new(1, 1),
					Some(mismatch)
				))
			);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hallo".chars(), None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output.len(), 1);
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn one_match_shortcut() {
			let parser = is('h').many1();
			let mut input = Input::new_from_chars("hallo".chars(), None);
			let output = parser(&mut input).unwrap();
			assert_eq!(output.len(), 1);
			assert_eq!(input.consumed_count(), 1);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}

		#[test]
		fn multiple_matches() {
			let token_count = 100;
			let parser = is('h');
			let tokens = std::iter::repeat_n('h', token_count).collect::<Vec<_>>();
			let mut input = Input::new_from_chars(tokens, None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output.len(), token_count);
			assert!(output.iter().all(|x| *x == 'h'));
			assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
		}

		#[test]
		fn partial_match_then_stop() {
			let parser = is('h');
			let mut input = Input::new_from_chars("hhjklmnop".chars(), None);
			let parser_many1 = many1(&parser);
			let output = parser_many1(&mut input).unwrap();
			assert_eq!(output, vec!['h', 'h']);
			assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
		}
	}
}
