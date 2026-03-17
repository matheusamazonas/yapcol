use crate::*;

#[test]
fn even_number() {
	let is_even_length = satisfy(|token: &&str| {
		if token.len().is_multiple_of(2) {
			Ok(<&str>::clone(token))
		} else {
			Err(Error::UnexpectedToken)
		}
	});
	// Even succeeds.
	let tokens = vec!["hello!"];
	let mut input = Input::new(tokens);
	assert_eq!(is_even_length(&mut input), Ok("hello!"));
	assert!(end_of_input()(&mut input).is_ok());
	// Odd number fails and does not consume.
	let tokens = vec!["hello", "hallo"];
	let mut input = Input::new(tokens);
	assert_eq!(is_even_length(&mut input), Err(Error::UnexpectedToken));
	assert_eq!(input.next_token(), Some("hello"));
	assert_eq!(input.next_token(), Some("hallo"));
}
