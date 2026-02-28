use crate::error::Error;
use crate::input::{Input, Token};

pub mod error;
#[cfg(test)]
mod tests;
pub mod input;

pub trait Parser<I, O>: Fn(&mut Input<I>) -> Result<O, Error> 
where
I: Iterator,
{}

impl<I, O, T> Parser<I, O> for T
where 
	I: Iterator,
	T: Fn(&mut Input<I>) -> Result<O, Error> { }

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
/// use yapcol_rs::input::Input;
///
/// let tokens: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut input = Input::new(tokens);
/// let parser = is(&'h');
/// assert!(parser(&mut input).is_ok());
///
/// let mut wrong: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// let mut input = Input::new(wrong);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(input.consumed_count(), 0); // Input was not consumed.
/// ```
pub fn is<I>(i: &I::Item) -> impl Parser<I, I::Item>
where
	I: Iterator,
	I::Item: PartialEq + Clone,
{
	let f = |x: &I::Item|
		match *x == *i {
			true => Ok((*x).clone()),
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
/// use yapcol_rs::input::Input;
///
/// let tokens: Vec<char> = vec!['3', 'a', 'b'];
/// let mut input = Input::new(tokens);
/// let parser = satisfy(|c: &char| {
///     if c.is_ascii_digit() { Ok(*c) } else { Err(Error::UnexpectedToken) }
/// });
/// assert_eq!(parser(&mut input).unwrap(), '3');
/// assert_eq!(input.consumed_count(), 1); // Token was consumed.
///
/// let tokens: Vec<char> = vec!['a', 'b', 'c'];
/// let mut input = Input::new(tokens);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(input.consumed_count(), 0); // Input was not consumed.
/// ```
pub fn satisfy<F, I, O>(f: F) -> impl Parser<I, O>
where
	F: Fn(&I::Item) -> Result<O, Error>,
	I: Iterator,
	I::Item: Token,
{
	move |input| match input.peek_next() {
		Some(token) => {
			match f(token) {
				Ok(result) => {
					input.next_token(); // Consume if successful.
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
/// use yapcol_rs::input::Input;
///
/// let tokens: Vec<char> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(end_of_input()(&mut input).is_ok());
///
/// let tokens: Vec<char> = vec!['a', 'b'];
/// let mut input = Input::new(tokens); 
/// assert!(end_of_input()(&mut input).is_err());
/// assert_eq!(input.consumed_count(), 0); // Input was not consumed.
/// ```
pub fn end_of_input<I>() -> impl Parser<I, ()> 
where
	I: Iterator,
	I::Item: Token,
{
	|input| match input.peek_next() {
		None => Ok(()),
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
/// use yapcol_rs::input::Input;
///
/// // parser1 succeeds: returns its result.
/// let tokens: Vec<char> = vec!['a', 'b'];
/// let mut input = Input::new(tokens);
/// let output = option(&is(&'a'), &is(&'b'))(&mut input).unwrap();
/// assert_eq!(output, 'a');
///
/// // parser1 fails without consuming input: parser2 is tried.
/// let tokens: Vec<char> = vec!['b'];
/// let mut input = Input::new(tokens);
/// let output = option(&is(&'a'), &is(&'b'))(&mut input).unwrap();
/// assert_eq!(output, 'b');
///
/// // Both parsers fail: error is returned and input is not consumed.
/// let tokens: Vec<char> = vec!['c'];
/// let mut input = Input::new(tokens);
/// assert!(option(&is(&'a'), &is(&'b'))(&mut input).is_err());
/// assert_eq!(input.consumed_count(), 0);
/// ```
pub fn option<P1, P2, I, O>(parser1: &P1, parser2: &P2) -> impl Parser<I, O>
where
	P1: Parser<I, O>,
	P2: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
{
	|input| {
		let initial_length = input.consumed_count();
		match parser1(input) {
			Ok(token) => Ok(token),
			Err(_) if input.consumed_count() == initial_length => parser2(input),
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
/// use yapcol_rs::input::Input;
///
/// let tokens: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut input = Input::new(tokens);
/// let ph = is(&'h');
/// let parser = maybe(&ph);
/// assert_eq!(parser(&mut input).unwrap(), Some('h'));
///
/// let tokens: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// let mut input = Input::new(tokens);
/// assert_eq!(parser(&mut input).unwrap(), None);
/// assert_eq!(input.consumed_count(), 0); // Input was not consumed.
/// ```
pub fn maybe<P, I, O>(parser: &P) -> impl Parser<I, Option<O>>
where
	P: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
{
	|input| {
		let initial_length = input.consumed_count();
		match parser(input) {
			Ok(token) => Ok(Some(token)),
			Err(_) if input.consumed_count() == initial_length => Ok(None),
			Err(e) => Err(e),
		}
	}
}

fn many<P, I, O>(parser: &P) -> impl Fn(&mut Input<I>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
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
/// use yapcol_rs::input::Input;
///
/// // Matches multiple elements
/// let parser = is(&1);
/// let tokens = vec![1, 1, 2];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![1, 1]));
///
/// // Returns empty vector when no matches are found (never fails)
/// let tokens: Vec<i32> = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
///
/// // Returns empty vector on empty input (never fails)
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
/// ```
pub fn many0<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
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
/// use yapcol_rs::input::Input;
///
/// // Matches multiple elements
/// let parser = is(&1);
/// let tokens = vec![1, 1, 2];
/// let mut input = Input::new(tokens);
/// assert_eq!(many1(&parser)(&mut input), Ok(vec![1, 1]));
///
/// // Fails when no matches are found
/// let tokens: Vec<i32> = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert!(many1(&parser)(&mut input).is_err());
///
/// // Fails on empty input
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(many1(&parser)(&mut input).is_err());
/// ```
pub fn many1<P, I, O>(parser: &P) -> impl Parser<I, Vec<O>>
where
	P: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
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
/// use yapcol_rs::input::Input;
///
/// // Returns the result of the first matching parser
/// let p1 = is(&1);
/// let p2 = is(&2);
/// let parsers = vec![p1, p2];
/// let tokens = vec![2, 3];
/// let mut input = Input::new(tokens); 
/// assert_eq!(choice(&parsers)(&mut input), Ok(2));
///
/// // Fails when no parser matches
/// let tokens = vec![3, 4];
/// let mut input = Input::new(tokens);
/// assert!(choice(&parsers)(&mut input).is_err());
///
/// // Fails on empty input
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(choice(&parsers)(&mut input).is_err());
/// ```
pub fn choice<'a, P, I, O, PI>(parsers: &'a PI) -> impl Parser<I, O>
where
	P: Parser<I, O> + 'a,
	I: Iterator,
	I::Item: Token,
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
/// use yapcol_rs::input::Input;
///
/// // Succeeds when the parser matches exactly `count` times
/// let parser = is(&1);
/// let tokens = vec![1, 1, 1, 2];
/// let mut input = Input::new(tokens);
/// assert_eq!(count(&parser, 3)(&mut input), Ok(vec![1, 1, 1]));
/// assert_eq!(input.next_token(), Some(2)); // Remaining input after consuming 3 tokens.
///
/// // Fails when there are not enough matching tokens
/// let tokens = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// assert!(count(&parser, 3)(&mut input).is_err());
///
/// // Succeeds with count = 0, returning an empty vector
/// let tokens = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// assert_eq!(count(&parser, 0)(&mut input), Ok(vec![]));
///
/// // Fails on empty input when count > 0
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(count(&parser, 1)(&mut input).is_err());
/// ```
pub fn count<P, I, O>(parser: &P, count: usize) -> impl Parser<I, Vec<O>>
where
	I: Iterator,
	I::Item: Token,
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
/// use yapcol_rs::input::Input;
///
/// // Succeeds and returns the matched value without consuming input
/// let tokens = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// let parser = is(&1);
/// assert_eq!(look_ahead(&parser)(&mut input), Ok(1));
/// assert_eq!(input.next_token(), Some(1)); // Input was not consumed.
/// assert_eq!(input.next_token(), Some(2)); // Input was not consumed.
/// assert_eq!(input.next_token(), Some(3)); // Input was not consumed.
///
/// // Can be applied multiple times since input is never consumed
/// assert_eq!(look_ahead(&parser)(&mut input), Ok(1));
///
/// // Fails without consuming input when the token does not match
/// let tokens = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert!(look_ahead(&parser)(&mut input).is_err());
/// assert_eq!(input.next_token(), Some(2)); // Input was not consumed.
/// assert_eq!(input.next_token(), Some(3)); // Input was not consumed.
///
/// // Fails on empty input
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(look_ahead(&parser)(&mut input).is_err());
/// ```
pub fn look_ahead<P, I, O>(parser: &P) -> impl Parser<I, O>
where
	P: Parser<I, O>,
	I: Iterator,
	I::Item: Token,
{
	|input| {
		// let mut next = input.peek();
		// parser(&mut next)
		todo!()
	}
}

// TO DO list:
// - between
// - separated by (0, 1)
// - chain left (0, 1)
// - chain right (0, 1)
// - any