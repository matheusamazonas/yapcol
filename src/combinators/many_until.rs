use crate::{InputToken, Parser};

/// Parses one or more instances of `parser`, until `end` succeeds.
///
/// # Arguments
///
/// - `parser`: the parser for the elements to be collected until the end is reached.
/// - `end`: the parser that delimits the end.
///
/// # Examples
///
/// ```
/// use yapcol::{Error, Input, any, is, many_until};
///
/// let comments_parser = |input: &mut Input<_>| {
/// 	let open = is('#');
/// 	let close = is('$');
/// 	let any = any();
/// 	open(input)?;
/// 	many_until(&any, &close)(input)
/// };
/// let mut input = Input::new_from_chars("#this is a comment$".chars(), None);
/// let output = comments_parser(&mut input);
/// assert_eq!(output, Ok("this is a comment".chars().collect()));
/// ```
pub fn many_until<P, PE, IT, O, OE>(parser: &P, end: &PE) -> impl Parser<IT, Vec<O>>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
{
	|input| {
		let mut matches = Vec::new();
		while end(input).is_err() {
			let token = parser(input)?;
			matches.push(token);
		}
		Ok(matches)
	}
}

#[cfg(test)]
mod tests {
	use crate::input::Position;
	use crate::*;

	#[test]
	fn empty() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(output, Err(Error::EndOfInput));
	}

	#[test]
	fn success_none() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("#".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input).unwrap();
		assert_eq!(output, Vec::<char>::new());
	}

	#[test]
	fn success_multiple() {
		let any_parser = any();
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("Hello world #".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input).unwrap();
		assert_eq!(output, "Hello world ".chars().collect::<Vec<_>>());
	}

	#[test]
	fn fail() {
		let any_parser = is('x');
		let end_comment_parser = is('#');
		let mut input = Input::new_from_chars("xxxxxy".chars(), None);
		let not_followed_parser = many_until(&any_parser, &end_comment_parser);
		let output = not_followed_parser(&mut input);
		assert_eq!(
			output,
			Err(Error::UnexpectedToken(None, Position::new(1, 6)))
		);
		assert_eq!(any()(&mut input), Ok('y')); // Input was consumed while looking for the end.
	}
}
