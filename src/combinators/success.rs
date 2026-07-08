use crate::{InputToken, Parser};

/// Creates a parser that always succeeds with the given value.
///
/// # Outcome
///
/// This combinator always succeeds.
///
/// # Input consumption
///
/// This combinator never consumes input.
///
/// # Look-ahead and backtracking
///
/// This combinator never performs lookahead.
///
/// # Arguments
///
/// - `success`: the value to return when the parser is applied.
///
/// # Examples
///
/// ```
/// use yapcol::{Input, any, success};
///
/// // Always succeeds, even on empty input.
/// let mut input = Input::new_from_chars("".chars(), None);
/// assert_eq!(success(1)(&mut input), Ok(1));
///
/// // Does not consume any input.
/// let mut input = Input::new_from_chars("hello".chars(), None);
/// assert_eq!(success(1)(&mut input), Ok(1));
/// assert_eq!(any()(&mut input), Ok('h'));
/// ```
pub fn success<IT, O>(success: O) -> impl Parser<IT, O>
where
	IT: InputToken,
	O: Clone,
{
	move |_| Ok(success.clone())
}
