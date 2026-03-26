//! YAPCoL (Yet Another Parser Combinator Library) is a flexible and simple-to-use
//! parser combinator library for Rust.
//!
//! It allows you to build complex parsers by combining smaller, simpler ones.
//! The library is designed to be straightforward, while still providing powerful features like
//! arbitrary lookahead and nested parsers.
//!
//! # Core Concepts
//!
//! - [`Parser`]: The central trait of the crate. Any function that takes a mutable reference
//!   to an [`Input`] and returns a `Result<Output, Error>` is a parser.
//! - [`Input`]: A wrapper around an iterator that provides buffering and lookahead capabilities.
//! - **Combinators**: Functions that take one or more parsers and return a new, more complex
//!   parser. Examples: [`is`], [`many0`], [`option`], [`chain_left`].
//!
//! # Features
//!
//! - **Arbitrary Lookahead**: Easily backtrack and try alternative parsers using [`attempt`] and
//!   [`look_ahead`].
//! - **Generic Input**: Works with any iterator whose items implement the [`Token`] trait.
//!
//! # Quick Start
//!
//! ```
//! use yapcol::input::Input;
//! use yapcol::{is, many0};
//!
//! let mut input = Input::new("aaab".chars());
//!
//! // Combine `is` and `many0` to parse multiple 'a's
//! let is_a = is('a');
//! let parser = many0(&is_a);
//!
//! let result = parser(&mut input);
//! assert_eq!(result, Ok(vec!['a', 'a', 'a']));
//! ```
//!
//! # Examples
//!
//! YAPCoL has two crates in the `examples` directory that demonstrate the library's capabilities.
//! Both of them implement the same application: a simple arithmetic expression parser and
//! evaluator. Each example uses a slightly different implementation to achieve the task:
//!   - `evaluate_expression_string` uses a parser that takes a stream of *characters* as input.
//!     This example parsers the input string directly into the custom `Expression` type.
//!   - `evaluate_expression_token` uses a parser that takes a stream of user-defined *tokens* as
//!     input. This example first performs lexical analysis (lexing) to turn the input string into
//!     a vector of tokens, then parsers the token stream into the custom `Expression` type.
//!
//! These two approaches reflect real-world usage of parsers, which might parse text directly or
//! perform lexical analysis beforehand. Check the `README` file in the `examples` directory for
//! more information.

use crate::error::Error;
use crate::input::{Input, Position, PositionToken, Token};

pub mod error;
pub mod input;
#[cfg(test)]
mod tests;

/// The core trait of the `yapcol` crate, representing a parser.
///
/// A `Parser` is a function (or any type that implements `Fn`) that takes a mutable reference
/// to an [`Input`] and returns a `Result` containing either the successfully parsed output
/// of type `O` or an [`Error`].
///
/// This trait is automatically implemented for any function with the signature
/// `Fn(&mut Input<I>) -> Result<O, Error>`.
///
/// # Type Parameters
///
/// - `PT`: The parser's input token, which implements the [`PositionToken`] trait.
/// - `O`: The type of the value produced by the parser on success.
///
/// # Examples
///
/// You can define a custom parser as a function:
///
/// ```
/// use std::str::Chars;
/// use yapcol::input::Input;
/// use yapcol::error::Error;
/// use yapcol::is;
///
/// fn my_uppercase_parser(input: &mut Input<Chars>) -> Result<char, Error> {
///    // You can use existing parsers inside your custom parser
///    is('A')(input)
/// }
///
/// let mut input = Input::new("Abc".chars());
/// assert_eq!(my_uppercase_parser(&mut input), Ok('A'));
/// ```
///
/// Most of the time, you will use the built-in combinators which return `impl Parser`:
///
/// ```
/// use yapcol::input::Input;
/// use yapcol::is;
///
/// let mut input = Input::new("Abc".chars());
/// let mut parser = is('A');
/// assert_eq!(parser(&mut input), Ok('A'));
/// ```
pub trait Parser<PT, O>: Fn(&mut Input<PT>) -> Result<O, Error>
where
	PT: PositionToken,
{
}

impl<PT, O, X> Parser<PT, O> for X
where
	X: Fn(&mut Input<PT>) -> Result<O, Error>,
	PT: PositionToken,
{
}

/// Creates a parser that succeeds if the next token in the input equals `token`.
///
/// If the token matches, it is consumed and returned. If the token does not match, the parser
/// fails without consuming any input.
///
/// # Arguments
///
/// - `token`: A reference to the token to match against.
///
/// # Examples
///
/// ```
/// use yapcol::{is, any};
/// use yapcol::input::Input;
///
/// let tokens: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut input = Input::new(tokens);
/// let parser = is('h');
/// assert!(parser(&mut input).is_ok());
///
/// let mut wrong: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// let mut input = Input::new(wrong);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('w')); // Input was not consumed.
/// ```
pub fn is<PT>(token: PT::Token) -> impl Parser<PT, PT::Token>
where
	PT: PositionToken<Token: Token>,
{
	let f = move |t: &PT::Token| match *t == token {
		true => Ok((*t).clone()),
		false => Err(Error::UnexpectedToken(Position::placeholder())),
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
/// - `f`: A predicate that takes a reference to a token and returns `Ok` on success or
///   `Err` on failure.
///
/// # Examples
///
/// ```
/// use yapcol::{satisfy, any};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let tokens: Vec<char> = vec!['3', 'a', 'b'];
/// let mut input = Input::new(tokens);
/// let parser = satisfy(|c: &char| {
///     if c.is_ascii_digit() { Ok(*c) } else { Err(Error::UnexpectedToken) }
/// });
/// assert_eq!(parser(&mut input).unwrap(), '3');
/// assert_eq!(any()(&mut input), Ok('a')); // Token was consumed.
///
/// let tokens: Vec<char> = vec!['a', 'b', 'c'];
/// let mut input = Input::new(tokens);
/// assert!(parser(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('a')); // Input was not consumed.
/// ```
pub fn satisfy<F, PT, O>(f: F) -> impl Parser<PT, O>
where
	F: Fn(&PT::Token) -> Result<O, Error>,
	PT: PositionToken<Token: Token>,
{
	move |input| match input.peek() {
		Some(pos_token) => {
			let token = pos_token.token();
			match f(token) {
				Ok(result) => {
					input.next_token(); // Consume if successful.
					Ok(result)
				}
				Err(Error::UnexpectedToken(_)) => Err(Error::UnexpectedToken(pos_token.position())),
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
/// use yapcol::{end_of_input, any};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let tokens: Vec<char> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(end_of_input()(&mut input).is_ok());
///
/// let tokens: Vec<char> = vec!['a', 'b'];
/// let mut input = Input::new(tokens);
/// assert!(end_of_input()(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('a')); // Input was not consumed.
/// ```
pub fn end_of_input<PT>() -> impl Parser<PT, ()>
where
	PT: PositionToken,
{
	|input| match input.peek() {
		None => Ok(()),
		Some(t) => Err(Error::UnexpectedToken(t.position())),
	}
}

/// Creates a parser based on two input parsers. It tries the first parser and falls back to the
/// second if the first fails without consuming input.
///
/// If `parser1` succeeds, its result is returned. If `parser1` fails without consuming any input,
/// `parser2` is applied and its result is returned. If `parser1` fails consuming input, the
/// error is propagated and no attempt to apply `parser2` is made.
///
///  If you would like to attempt to apply `parser2` even if `parser1` failed consuming input,
/// check the `attempt` parser combinator.
///
/// # Arguments
///
/// - `parser1`: The first parser to try.
/// - `parser2`: The fallback parser, applied only if `parser1` fails without consuming input.
///
/// # Examples
///
/// ```
/// use yapcol::{option, is, end_of_input, any};
/// use yapcol::input::Input;
///
/// // parser1 succeeds: returns its result.
/// let tokens: Vec<char> = vec!['a', 'b'];
/// let mut input = Input::new(tokens);
/// let output = option(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'a');
///
/// // parser1 fails without consuming input: parser2 is tried.
/// let tokens: Vec<char> = vec!['b'];
/// let mut input = Input::new(tokens);
/// let output = option(&is('a'), &is('b'))(&mut input).unwrap();
/// assert_eq!(output, 'b');
///
/// // Both parsers fail: an error is returned and input is not consumed.
/// let tokens: Vec<char> = vec!['c'];
/// let mut input = Input::new(tokens);
/// assert!(option(&is('a'), &is('b'))(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok('c'));
/// ```
pub fn option<P1, P2, PT, O>(parser1: &P1, parser2: &P2) -> impl Parser<PT, O>
where
	P1: Parser<PT, O>,
	P2: Parser<PT, O>,
	PT: PositionToken,
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
/// - `parser`: The parser to make optional.
///
/// # Examples
///
/// ```
/// use yapcol::{maybe, is, any};
/// use yapcol::input::Input;
///
/// let tokens: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
/// let mut input = Input::new(tokens);
/// let ph = is('h');
/// let parser = maybe(&ph);
/// assert_eq!(parser(&mut input).unwrap(), Some('h'));
///
/// let tokens: Vec<char> = vec!['w', 'o', 'r', 'l', 'd'];
/// let mut input = Input::new(tokens);
/// assert_eq!(parser(&mut input).unwrap(), None);
/// assert_eq!(any()(&mut input), Ok('w')); // Input was not consumed.
/// ```
pub fn maybe<P, PT, O>(parser: &P) -> impl Parser<PT, Option<O>>
where
	P: Parser<PT, O>,
	PT: PositionToken,
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

fn many<P, PT, O>(parser: &P) -> impl Fn(&mut Input<PT>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<PT, O>,
	PT: PositionToken,
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
/// # Arguments
///
/// - `parser`: The parser to possibly be applied many times.
///
/// # Examples
///
/// ```
/// use yapcol::{many0, is};
/// use yapcol::input::Input;
///
/// // Matches multiple elements
/// let parser = is(1);
/// let tokens = vec![1, 1, 2];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![1, 1]));
///
/// // Returns an empty vector when no matches are found (never fails)
/// let tokens: Vec<i32> = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
///
/// // Returns an empty vector on empty input (never fails)
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert_eq!(many0(&parser)(&mut input), Ok(vec![]));
/// ```
pub fn many0<P, PT, O>(parser: &P) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	PT: PositionToken,
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
/// use yapcol::{many1, is};
/// use yapcol::input::Input;
///
/// // Matches multiple elements
/// let parser = is(1);
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
pub fn many1<P, PT, O>(parser: &P) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	PT: PositionToken,
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
/// # Arguments
///
/// - `parsers`: An iterator that contains all parsers to attempt until a success.
///
/// # Examples
///
/// ```
/// use yapcol::{choice, is};
/// use yapcol::input::Input;
///
/// // Returns the result of the first matching parser
/// let p1 = is(1);
/// let p2 = is(2);
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
pub fn choice<'a, P, PT, O, PI>(parsers: &'a PI) -> impl Parser<PT, O>
where
	P: Parser<PT, O> + 'a,
	PT: PositionToken,
	&'a PI: IntoIterator<Item = &'a P>,
{
	|input| {
		parsers
			.into_iter()
			.find_map(|p| p(input).ok())
			.ok_or(input.get_position_error())
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
/// - `parser`: The parser to apply repeatedly.
/// - `count`: The exact number of times to apply the parser.
///
/// # Examples
///
/// ```
/// use yapcol::{count, is, any};
/// use yapcol::input::Input;
///
/// // Succeeds when the parser matches exactly `count` times
/// let parser = is(1);
/// let tokens = vec![1, 1, 1, 2];
/// let mut input = Input::new(tokens);
/// assert_eq!(count(&parser, 3)(&mut input), Ok(vec![1, 1, 1]));
/// assert_eq!(any()(&mut input), Ok(2)); // Remaining input after consuming 3 tokens.
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
pub fn count<P, PT, O>(parser: &P, count: usize) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	PT: PositionToken,
{
	move |input| {
		let mut output = Vec::with_capacity(count);
		for _ in 0..count {
			match parser(input) {
				Ok(token) => output.push(token),
				Err(e) => return Err(e),
			}
		}
		Ok(output)
	}
}

/// Creates a parser that does not consume input in case the given parser succeeds.
///
/// If the given parser succeeds, the matched value is returned, but the input is left unchanged.
/// If the given parser fails consuming input, this parser also fails consuming input.
///
/// # Arguments
///
/// - `parser`: The parser to look ahead.
///
/// # Examples
///
/// ```
/// use yapcol::{look_ahead, is, end_of_input, any};
/// use yapcol::input::Input;
/// use yapcol::error::Error;
///
/// // Succeeds without consuming input.
/// let tokens = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// let parser1 = is(1);
/// assert_eq!(look_ahead(&parser1)(&mut input), Ok(1));
/// assert_eq!(any()(&mut input), Ok(1)); // Input was not consumed.
/// assert_eq!(any()(&mut input), Ok(2)); // any()(&mut input)Input was not consumed.
/// assert_eq!(any()(&mut input), Ok(3)); // Input was not consumed.
///
/// // Fails without consuming input.
/// let tokens = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert!(look_ahead(&parser1)(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok(2)); // Input was not consumed.
///
/// // Fails consuming input if the parser consumes.
/// let tokens = vec![1, 3];
/// let mut input = Input::new(tokens);
/// let consuming_parser = |input: &mut Input<_>| {
///     let o1 = parser1(input)?;
///     let o2 = parser1(input)?;
///     Ok((o1, o2))
/// };
/// let output = look_ahead(&consuming_parser)(&mut input);
/// assert_eq!(output, Err(Error::UnexpectedToken));
/// assert_eq!(any()(&mut input), Ok(3)); // Input was consumed.
///
/// // Fails on empty input.
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(look_ahead(&parser1)(&mut input).is_err());
/// ```
pub fn look_ahead<P, PT, O>(parser: &P) -> impl Parser<PT, O>
where
	P: Parser<PT, O>,
	PT: PositionToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, output.is_ok());
		output
	}
}

/// Creates a parser that does not consume input in case the given parser fails.
///
/// If the given parser succeeds, the matched value is returned. If the given parser consumed input,
/// this parser also does.
/// If the given parser fails consuming input, this parser also fails, but does not consume input.
///
/// This combinator is often used alongside [`option`] whenever both input parsers share a prefix. By
/// doing so, we prevent [`option`] from failing if its first parser argument failed while consuming
/// input. For example:
/// ```rust,ignore
/// // Instead of this, where `option` would fail early and not even try applying `parser2`.
/// let parser = option(&parser1, &parser2);
/// // Do this, so if `parser1` fails consuming input, `parser2` will be applied.
/// let attempt_parser_1 = attempt(&parser1);
/// let parser = option(&attempt_parser_1, &parser2);
/// ```
///
/// Warning: this combinator implements arbitrary lookahead.
///
/// # Arguments
///
/// - `parser`: The parser to attempt.
///
/// # Examples
///
/// ```
/// use yapcol::{attempt, is, end_of_input, any};
/// use yapcol::input::Input;
/// use yapcol::error::Error;
///
/// // Succeeds consuming input.
/// let tokens = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// let parser1 = is(1);
/// assert_eq!(attempt(&parser1)(&mut input), Ok(1));
/// assert_eq!(any()(&mut input), Ok(2)); // Input was consumed.
///
/// // Fails without consuming input.
/// let tokens = vec![2, 3];
/// let mut input = Input::new(tokens);
/// assert!(attempt(&parser1)(&mut input).is_err());
/// assert_eq!(any()(&mut input), Ok(2)); // Input was not consumed.
/// assert_eq!(any()(&mut input), Ok(3)); // Input was not consumed.
///
/// // Fails without consuming input if the parser consumes.
/// let tokens = vec![1, 3];
/// let mut input = Input::new(tokens);
/// let consuming_parser = |input: &mut Input<_>| {
///     let o1 = parser1(input)?;
///     let o2 = parser1(input)?;
///     Ok((o1, o2))
/// };
/// let output = attempt(&consuming_parser)(&mut input);
/// assert_eq!(output, Err(Error::UnexpectedToken));
/// assert_eq!(any()(&mut input), Ok(1)); // Input was not consumed.
///
/// // Fails on empty input
/// let tokens: Vec<i32> = vec![];
/// let mut input = Input::new(tokens);
/// assert!(attempt(&parser1)(&mut input).is_err());
/// ```
pub fn attempt<P, PT, O>(parser: &P) -> impl Parser<PT, O>
where
	P: Parser<PT, O>,
	PT: PositionToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, output.is_err());
		output
	}
}

/// Applies parsers `open` and `close` around `parser`. Often used for parenthesis, brackets, etc.
///
/// # Arguments
///
/// - `open`: The parser that "opens" the between.
/// - `parser`: The parser that goes between `open` and `close`, whose content we're interested in.
/// - `close`: The parser that "closes" the between.
///
/// # Examples
///
/// ```
/// use yapcol::{is, between};
/// use yapcol::input::Input;
///
/// let tokens: Vec<i32> = vec![1, 2, 1];
/// let mut input = Input::new(tokens);
/// let parser1 = is(1);
/// let parser2 = is(2);
/// let output = between(&parser1, &parser2, &parser1)(&mut input);
/// assert_eq!(output, Ok(2));
/// ```
pub fn between<PO, PC, P, PT, O, OO, OC>(open: &PO, parser: &P, close: &PC) -> impl Parser<PT, O>
where
	PO: Parser<PT, OO>,
	PC: Parser<PT, OC>,
	P: Parser<PT, O>,
	PT: PositionToken,
{
	move |input| {
		open(input)?;
		let output = parser(input)?;
		close(input)?;
		Ok(output)
	}
}

/// A simple combinator that returns the next token in the input, if any.
///
/// # Examples
///
/// ```
/// use yapcol::{any};
/// use yapcol::input::Input;
///
/// // An example input iterator
/// let tokens: Vec<i32> = vec![1, 2, 3];
/// let mut input = Input::new(tokens);
/// let output = any()(&mut input);
/// assert_eq!(output, Ok(1));
/// ```
pub fn any<PT>() -> impl Parser<PT, PT::Token>
where
	PT: PositionToken,
{
	|input| match input.next_token() {
		Some(pos_token) => Ok(pos_token.token_owned()),
		None => Err(Error::EndOfInput),
	}
}

fn separated_tail<P, S, PT, O, SO>(
	parser: &P,
	separator: &S,
) -> impl Fn(&mut Input<PT>, Vec<O>) -> Result<Vec<O>, Error>
where
	P: Parser<PT, O>,
	S: Parser<PT, SO>,
	PT: PositionToken,
{
	move |input, mut output| {
		while separator(input).is_ok() {
			let next = parser(input)?;
			output.push(next);
		}
		Ok(output)
	}
}

/// Creates a parser that parsers zero or more occurrences of `parser`, separated by `separator`.
///
/// # Arguments
///
/// - `parser`: The parser whose occurrences we're collecting.
/// - `separator`: The separator parser, whose content we're not interested in.
///
/// # Examples
///
/// ```
/// use yapcol::{is, separated_by0};
/// use yapcol::input::Input;
///
/// let parser1 = is(1);
/// let parser2 = is(2);
/// let tokens = vec![1, 2, 1];
/// let mut input = Input::new(tokens);
/// let parser_separated_by0 = separated_by0(&parser1, &parser2);
/// let output = parser_separated_by0(&mut input);
/// assert_eq!(output, Ok(vec![1, 1]));
/// ```
pub fn separated_by0<P, S, PT, O, OS>(parser: &P, separator: &S) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	S: Parser<PT, OS>,
	PT: PositionToken,
{
	move |input| match parser(input) {
		Ok(token) => {
			let output = vec![token];
			separated_tail(&parser, &separator)(input, output)
		}
		Err(Error::EndOfInput) => Ok(vec![]),
		Err(_) => Ok(vec![]),
	}
}

/// Creates a parser that parsers one or more occurrences of `parser`, separated by `separator`.
///
/// # Arguments
///
/// - `parser`: The parser whose occurrences we're collecting.
/// - `separator`: The separator parser, whose content we're not interested in.
///
/// # Examples
///
/// ```
/// use yapcol::{is, separated_by1};
/// use yapcol::input::Input;
///
/// let parser1 = is(1);
/// let parser2 = is(2);
/// let tokens = vec![1, 2, 1];
/// let mut input = Input::new(tokens);
/// let parser_separated_by1 = separated_by1(&parser1, &parser2);
/// let output = parser_separated_by1(&mut input);
/// assert_eq!(output, Ok(vec![1, 1]));
/// ```
pub fn separated_by1<P, S, PT, O, SO>(parser: &P, separator: &S) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	S: Parser<PT, SO>,
	PT: PositionToken,
{
	move |input| {
		let first = parser(input)?;
		let output = vec![first];
		separated_tail(&parser, &separator)(input, output)
	}
}

fn parse_chain_left_tail<P, PT, O, OP, F>(
	o1: O,
	parser: &P,
	operator_parser: &OP,
) -> impl Parser<PT, O>
where
	P: Parser<PT, O>,
	PT: PositionToken,
	OP: Parser<PT, F>,
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
/// `operator_parser`, in a left-associative manner.
///
/// # Arguments
///
/// - `operand_parser`: Parsers operands that will be combined into a final value, in a
///   left-associative manner.
/// - `operator_parser`: Operator's parser, which consumes input and returns a function that
///   combines output values into one.
///
/// # Examples
///
/// ```
/// // Implements evaluation of the subtraction ('-') operator as left-associative.
/// use yapcol::{satisfy, chain_left};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let operand = satisfy(|c: &char| match c.to_digit(10)  {
///     Some(x) => Ok(x as i32),
///     None => Err(Error::UnexpectedToken)
/// });
///
/// let operator = satisfy(|c: &char| match c {
///     '-' => Ok(|a, b| a - b),
///     _ => Err(Error::UnexpectedToken),
/// });
///
/// let tokens: Vec<_> = "3-1-1".chars().collect();
/// let mut input = Input::new(tokens);
/// let parser = chain_left(&operand, &operator);
/// let output = parser(&mut input);
/// assert_eq!(output, Ok(1)); // (3 - 1) - 1 = 1, not 3 - (1 - 1) = 3
/// ```
pub fn chain_left<P, PT, O, OP, F>(operand_parser: &P, operator_parser: &OP) -> impl Parser<PT, O>
where
	P: Parser<PT, O>,
	PT: PositionToken,
	OP: Parser<PT, F>,
	F: Fn(O, O) -> O,
	O: Clone,
{
	move |input| {
		let o1 = operand_parser(input)?;
		parse_chain_left_tail(o1, operand_parser, operator_parser)(input)
	}
}

/// Parses at least one occurrence of `operand_parser`, separated by `operator_parser`. It combines
/// all values parsed by `operand_parser` into a final one using functions returned by
/// `operator_parser`, in a right-associative manner.
///
/// # Arguments
///
/// - `operand_parser`: Parsers operands that will be combined into a final value, in a
///   right-associative manner.
/// - `operator_parser`: Operator's parser, which consumes input and returns a function that
///   combines output values into one.
///
/// # Examples
///
/// ```
/// // Implements evaluation of the subtraction ('-') operator as left-associative.
/// use yapcol::{satisfy, chain_right};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let operand = satisfy(|c: &char| match c.to_digit(10)  {
///     Some(x) => Ok(x as i32),
///     None => Err(Error::UnexpectedToken)
/// });
///
/// let operator = satisfy(|c: &char| match c {
///     '-' => Ok(|a, b| a - b),
///     _ => Err(Error::UnexpectedToken),
/// });
///
/// let tokens: Vec<_> = "3-1-1".chars().collect();
/// let mut input = Input::new(tokens);
/// let parser = chain_right(&operand, &operator);
/// let output = parser(&mut input);
/// assert_eq!(output, Ok(3)); // 3 - (1 - 1) = 3, not (3 - 1) - 1 = 1
/// ```
pub fn chain_right<P, PT, O, OP, F>(operand_parser: &P, operator_parser: &OP) -> impl Parser<PT, O>
where
	P: Parser<PT, O>,
	PT: PositionToken,
	OP: Parser<PT, F>,
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

/// Succeeds if `parser` fails. This combinator does not consume input, even if `parser` does.
///
/// # Arguments
///
/// - `parser`: the parser which should fail for this combinator to succeed.
///
/// # Examples
///
/// ```
/// use yapcol::{is, not_followed_by};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let parser = is("hello");
/// let tokens: Vec<&str> = vec!["world"];
/// let mut input = Input::new(tokens);
/// let not_followed_parser = not_followed_by(&parser);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(output, Ok(()));
///
/// let tokens: Vec<&str> = vec!["hello"];
/// let mut input = Input::new(tokens);
/// let output = not_followed_parser(&mut input);
/// assert_eq!(output, Err(Error::UnexpectedToken));
///
/// ```
pub fn not_followed_by<P, PT, O>(parser: &P) -> impl Parser<PT, ()>
where
	P: Parser<PT, O>,
	PT: PositionToken,
{
	|input| {
		let handler = input.start_look_ahead();
		let output = parser(input);
		input.stop_look_ahead(handler, true);
		match output {
			Ok(_) => Err(input.get_position_error()),
			Err(Error::EndOfInput) => Err(Error::EndOfInput),
			Err(_) => Ok(()),
		}
	}
}

/// Parsers one or more instances of `parser`, until `end` succeeds.
///
/// # Arguments
///
/// - `parser`: the parser for the elements to be collected until the end is reached.
/// - `end`: the parser that delimits the end.
///
/// # Examples
///
/// ```
/// use yapcol::{any, is, many_until};
/// use yapcol::error::Error;
/// use yapcol::input::Input;
///
/// let comments_parser = |input: &mut Input<_>| {
///     let open = is("/*");
///     let close = is("*/");
///     let any = any();
///     open(input)?;
///     many_until(&any, &close)(input)
/// };
/// let tokens: Vec<&str> = vec!["/*", "this", "is", "a", "comment", "*/"];
/// let mut input = Input::new(tokens);
/// let output = comments_parser(&mut input);
/// assert_eq!(output, Ok(vec!["this", "is", "a", "comment"]));
/// ```
pub fn many_until<P, PE, PT, O, OE>(parser: &P, end: &PE) -> impl Parser<PT, Vec<O>>
where
	P: Parser<PT, O>,
	PE: Parser<PT, OE>,
	PT: PositionToken,
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
