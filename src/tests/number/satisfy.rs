use crate::*;

#[test]
fn parse_satisfy_even_number() {
	let even = satisfy(|x: &i32| if x % 2 == 0 { Ok(*x) } else { Err(Error::UnexpectedToken) });
	// Even succeeds.
	let tokens = vec![4];
	let mut input = Input::new(tokens);
	assert_eq!(even(&mut input), Ok(4));
	assert!(end_of_input()(&mut input).is_ok());
	// Odd number fails and does not consume.
	let tokens = vec![5, 4];
	let mut input = Input::new(tokens);
	assert_eq!(even(&mut input), Err(Error::UnexpectedToken));
	assert_eq!(input.next_token(), Some(5));
	assert_eq!(input.next_token(), Some(4));
}
