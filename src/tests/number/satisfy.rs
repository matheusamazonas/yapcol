use crate::*;

#[test]
fn parse_satisfy_even_number() {
	let even = satisfy(|x: &i32| if x % 2 == 0 { Ok(*x) } else { Err(Error::UnexpectedToken) });
	// Even succeeds.
	let mut tokens = vec![4];
	assert_eq!(even(&mut tokens), Ok(4));
	assert!(end_of_input()(&mut tokens).is_ok());
	// Odd number fails and does not consume.
	let mut tokens = vec![5, 4];
	assert_eq!(even(&mut tokens), Err(Error::UnexpectedToken));
	assert_eq!(tokens, vec![5, 4]);
}
