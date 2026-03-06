use crate::*;

#[test]
fn empty() {
	let tokens: Vec<i32> = Vec::new();
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn success_does_not_consume() {
	let tokens = vec![1, 2, 3];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Ok(1));
	// After look_ahead, input should still start with 1
	assert_eq!(is(1)(&mut input), Ok(1));
	assert_eq!(is(2)(&mut input), Ok(2));
	assert_eq!(is(3)(&mut input), Ok(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn non_consuming_fail_does_not_consume() {
	let tokens = vec![2, 3];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input should still be intact.
	assert_eq!(input.next_token(), Some(2));
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn consuming_fail_consumes() {
	let tokens = vec![2, 3];
	let mut input = Input::new(tokens);
	let parser = |input: &mut Input<_>| {
		let output1 = is(2)(input)?; // Success, therefore it consumed.
		let output2 = is(1)(input)?; // Failed, so the whole parser fails consuming.
		Ok((output1, output2))
	};
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
	// Input was consumed.
	assert_eq!(input.next_token(), Some(3));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn look_ahead_twice() {
	let tokens = vec![1];
	let mut input = Input::new(tokens);
	let parser = is(1);
	let first = look_ahead(&parser)(&mut input);
	let second = look_ahead(&parser)(&mut input);
	assert_eq!(first, Ok(1));
	assert_eq!(second, Ok(1));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}
