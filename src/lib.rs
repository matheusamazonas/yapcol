use crate::error::Error;
use crate::input::InputStream;

pub mod error;
#[cfg(test)]
mod tests;
mod input;

pub trait Parser<I, O>: Fn(&mut I) -> Result<O, Error> 
where
I: InputStream,
{}

impl<I, O, T> Parser<I, O> for T 
where 
	I: InputStream,
	T: Fn(&mut I) -> Result<O, Error> { }


/// Creates a parser that succeeds if the next token in the input equals `i`.
///
/// If the token matches, it is consumed and returned. If the token does not match, the parser 
/// fails without consuming any input.
///
/// # Arguments
///
/// * `i` - A reference to the token to match against.
///
/// # Examples
///
/// ```
/// use yapcol_rs::is;
///
/// let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let parser = is(&'h');
/// assert!(parser(&mut input).is_ok());
///
/// let mut wrong: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// assert!(parser(&mut wrong).is_err());
/// assert_eq!(wrong.len(), 5); // Input was not consumed.
/// ```
pub fn is<I, T>(i: &T) -> impl Parser<I, T>
where
	I: InputStream<Token = T>,
	T: PartialEq + Clone,
{
	let f = |x: &T| match *x == *i {
		true => Ok(x.clone()),
		false => Err(Error::UnexpectedToken),
	};
	satisfy(f)
}

/// Creates a parser that succeeds if the given predicate returns `Ok` for the next token.
///
/// If the predicate succeeds, the token is consumed and the result is returned. If the predicate
/// fails, the parser fails without consuming any input.
///
/// # Arguments
///
/// * `f` - A predicate that takes a reference to a token and returns `Ok` on success or 
///   `Err` on failure.
///
/// # Examples
///
/// ```
/// use yapcol_rs::satisfy;
/// use yapcol_rs::error::Error;
///
/// let mut input: Vec<char> = vec!['3', 'a', 'b'];
/// let parser = satisfy(|c: &char| {
///     if c.is_ascii_digit() { Ok(*c) } else { Err(Error::UnexpectedToken) }
/// });
/// assert_eq!(parser(&mut input).unwrap(), '3');
/// assert_eq!(input.len(), 2); // Token was consumed.
///
/// let mut input: Vec<char> = vec!['a', 'b', 'c'];
/// assert!(parser(&mut input).is_err());
/// assert_eq!(input.len(), 3); // Input was not consumed.
/// ```
pub fn satisfy<F, I, O, T>(f: F) -> impl Parser<I, O>
where
	F: Fn(&T) -> Result<O, Error>,
	I: InputStream<Token = T>,
{
	move |input| match input.next_as_ref() {
		Some(token) => {
			match f(token) {
				Ok(result) => {
					input.remove_next(); // Consume if successful.
					Ok(result)
				}
				Err(e) => Err(e),
			}
		}
		None => Err(Error::EndOfInput),
	}
}


/// Creates a parser that succeeds only if the input stream is empty.
///
/// If the input is empty, the parser succeeds and returns `()`. If the input still has tokens,
/// the parser fails without consuming any input.
///
/// # Examples
///
/// ```
/// use yapcol_rs::end_of_input;
///
/// let mut empty: Vec<char> = vec![];
/// assert!(end_of_input()(&mut empty).is_ok());
///
/// let mut input: Vec<char> = vec!['a', 'b'];
/// assert!(end_of_input()(&mut input).is_err());
/// assert_eq!(input.len(), 2); // Input was not consumed.
/// ```
pub fn end_of_input<I>() -> impl Parser<I, ()> 
where
	I: InputStream,
{
	|input| match input.len() {
		0 => Ok(()),
		_ => Err(Error::UnexpectedToken),
	}
}


/// Creates a parser based on two input parsers. It tries the first parser and falls back to the
/// second if the first fails without consuming input.
///
/// If `parser1` succeeds, its result is returned. If `parser1` fails without consuming any input,
/// `parser2` is applied and its result is returned. If `parser1` fails consuming input, the
/// error is propagated and no attempt to apply `parser2` is made.
///
/// # Arguments
///
/// * `parser1` - The first parser to try.
/// * `parser2` - The fallback parser, applied only if `parser1` fails without consuming input.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{option, is, end_of_input};
///
/// // parser1 succeeds: returns its result.
/// let mut input: Vec<char> = vec!['a', 'b'];
/// let output = option(&is(&'a'), &is(&'b'))(&mut input).unwrap();
/// assert_eq!(output, 'a');
///
/// // parser1 fails without consuming input: parser2 is tried.
/// let mut input: Vec<char> = vec!['b'];
/// let output = option(&is(&'a'), &is(&'b'))(&mut input).unwrap();
/// assert_eq!(output, 'b');
///
/// // Both parsers fail: error is returned and input is not consumed.
/// let mut input: Vec<char> = vec!['c'];
/// assert!(option(&is(&'a'), &is(&'b'))(&mut input).is_err());
/// assert_eq!(input.len(), 1);
/// ```
pub fn option<P1, P2, I, O>(parser1: &P1, parser2: &P2) -> impl Parser<I, O>
where
	P1: Parser<I, O>,
	P2: Parser<I, O>,
	I: InputStream,
{
	|input| {
		let initial_length = input.len();
		match parser1(input) {
			Ok(token) => Ok(token),
			Err(_) if input.len() == initial_length => parser2(input),
			Err(e) => Err(e),
		}
	}
}

/// Creates a parser that makes another parser optional.
///
/// If the input parser succeeds, its result is wrapped in `Some` and returned. If the input
/// parser fails **without consuming any input**, the returned parser succeeds with `Ok(None)`.
/// If the input parser fails **after consuming input**, the error is propagated as `Err`.
///
/// # Arguments
///
/// * `parser` - The parser to make optional.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{maybe, is};
///
/// let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let ph = is(&'h');
/// let parser = maybe(&ph);
/// assert_eq!(parser(&mut input).unwrap(), Some('h'));
///
/// let mut input: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// assert_eq!(parser(&mut input).unwrap(), None);
/// assert_eq!(input.len(), 5); // Input was not consumed.
/// ```
pub fn maybe<P, I, O>(parser: &P) -> impl Parser<I, Option<O>>
where
	P: Parser<I, O>,
	I: InputStream,
{
	|input| {
		let initial_length = input.len();
		match parser(input) {
			Ok(token) => Ok(Some(token)),
			Err(_) if input.len() == initial_length => Ok(None),
			Err(e) => Err(e),
		}
	}
}

fn many<P, I, O>(parser: &P) -> impl Fn(&mut I, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<I, O>,
	I: InputStream,
{
	|input, mut output| match parser(input) {
		Ok(token) => {
			output.push(token);
			many(parser)(input, output)
		}
		Err(_) => Ok(output),
	}
}

/// Applies `parser` zero or more times, returning a vector of matches.
/// This parser never fails: if no matches are found, it returns an empty vector.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{many0, is};
///
/// // Matches multiple elements
/// let parser = is(&1);
/// let mut tokens = vec![1, 1, 2];
/// assert_eq!(many0(&parser)(&mut tokens), Ok(vec![1, 1]));
///
/// // Returns empty vector when no matches are found (never fails)
/// let mut tokens: Vec<i32> = vec![2, 3];
/// assert_eq!(many0(&parser)(&mut tokens), Ok(vec![]));
///
/// // Returns empty vector on empty input (never fails)
/// let mut tokens: Vec<i32> = vec![];
/// assert_eq!(many0(&parser)(&mut tokens), Ok(vec![]));
/// ```
pub fn many0<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
	I: InputStream,
{
	|input| {
		let output: Vec<O> = Vec::new();
		many(parser)(input, output)
	}
}

/// Applies `parser` one or more times, returning a vector of matches.
/// This parser fails if no matches are found.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{many1, is};
///
/// // Matches multiple elements
/// let parser = is(&1);
/// let mut tokens = vec![1, 1, 2];
/// assert_eq!(many1(&parser)(&mut tokens), Ok(vec![1, 1]));
///
/// // Fails when no matches are found
/// let mut tokens: Vec<i32> = vec![2, 3];
/// assert!(many1(&parser)(&mut tokens).is_err());
///
/// // Fails on empty input
/// let mut tokens: Vec<i32> = vec![];
/// assert!(many1(&parser)(&mut tokens).is_err());
/// ```
pub fn many1<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
	I: InputStream,
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

/// Applies each parser in `parsers` in order, returning the result of the first one that succeeds.
/// Fails if all parsers fail.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{choice, is};
///
/// // Returns the result of the first matching parser
/// let p1 = is(&1);
/// let p2 = is(&2);
/// let parsers = vec![p1, p2];
/// let mut tokens = vec![2, 3];
/// assert_eq!(choice(&parsers)(&mut tokens), Ok(2));
///
/// // Fails when no parser matches
/// let mut tokens = vec![3, 4];
/// assert!(choice(&parsers)(&mut tokens).is_err());
///
/// // Fails on empty input
/// let mut tokens: Vec<i32> = vec![];
/// assert!(choice(&parsers)(&mut tokens).is_err());
/// ```
pub fn choice<'a, P, I, O, PI>(parsers: &'a PI) -> impl Parser<I, O>
where
	P: Parser<I, O> + 'a,
	I: InputStream,
	&'a PI: IntoIterator<Item = &'a P>,
{
	|input| {
		parsers
			.into_iter()
			.find_map(|p| p(input).ok())
			.ok_or(Error::UnexpectedToken)
	}
}

/// Creates a parser that applies the given parser exactly `count` times.
///
/// The parser succeeds only if the given parser succeeds exactly `count` times in a row,
/// returning a vector of the matched values. If the given parser fails at any point before
/// `count` applications, the whole parser fails.
///
/// # Arguments
///
/// * `parser` - The parser to apply repeatedly.
/// * `count` - The exact number of times to apply the parser.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{count, is};
///
/// // Succeeds when the parser matches exactly `count` times
/// let parser = is(&1);
/// let mut input = vec![1, 1, 1, 2];
/// assert_eq!(count(&parser, 3)(&mut input), Ok(vec![1, 1, 1]));
/// assert_eq!(input, vec![2]); // Remaining input after consuming 3 tokens.
///
/// // Fails when there are not enough matching tokens
/// let mut input = vec![1, 2, 3];
/// assert!(count(&parser, 3)(&mut input).is_err());
///
/// // Succeeds with count = 0, returning an empty vector
/// let mut input = vec![1, 2, 3];
/// assert_eq!(count(&parser, 0)(&mut input), Ok(vec![]));
///
/// // Fails on empty input when count > 0
/// let mut input: Vec<i32> = vec![];
/// assert!(count(&parser, 1)(&mut input).is_err());
/// ```
pub fn count<P, I, O>(parser: &P, count: usize) -> impl Parser<I, Vec<O>>
where
	I: InputStream,
	P: Parser<I, O>,
{
	move |input| {
		let mut output = Vec::with_capacity(count);
		for _ in 0..count {
			match parser(input) {
				Ok(token) => output.push(token),
				Err(_) => return Err(Error::UnexpectedToken)
			}
		}
		Ok(output)
	}
}

/// Creates a parser that applies the given parser to the next token without consuming any input.
///
/// The parser is applied only to the next token (and not beyond). If the parser succeeds, the
/// matched value is returned, but the input is left unchanged. If the parser fails, the input is
/// also left unchanged.
///
/// # Arguments
///
/// * `parser` - The parser to apply to the next token.
///
/// # Examples
///
/// ```
/// use yapcol_rs::{look_ahead, is, end_of_input};
///
/// // Succeeds and returns the matched value without consuming input
/// let mut input = vec![1, 2, 3];
/// let parser = is(&1);
/// assert_eq!(look_ahead(&parser)(&mut input), Ok(1));
/// assert_eq!(input, vec![1, 2, 3]); // Input was not consumed.
///
/// // Can be applied multiple times since input is never consumed
/// assert_eq!(look_ahead(&parser)(&mut input), Ok(1));
///
/// // Fails without consuming input when the token does not match
/// let mut input = vec![2, 3];
/// assert!(look_ahead(&parser)(&mut input).is_err());
/// assert_eq!(input, vec![2, 3]); // Input was not consumed.
///
/// // Fails on empty input
/// let mut input: Vec<i32> = vec![];
/// assert!(look_ahead(&parser)(&mut input).is_err());
/// ```
pub fn look_ahead<P, I, O>(parser: &P) -> impl Parser<I, O>
where
	P: Parser<I, O>,
	I: InputStream,
{
	|input| {
		let mut next = input.peek();
		parser(&mut next)
	}
}
// TO DO list:
// - between
// - separated by (0, 1)
// - chain left (0, 1)
// - chain right (0, 1)
// - any