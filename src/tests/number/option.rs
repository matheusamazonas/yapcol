use crate::*;

#[test]
fn parse_option_first() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![1];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Ok(1));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_second() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![2];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Ok(2));
	assert!(end_of_input()(&mut tokens).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_option_none() {
	let parser1 = is(&1);
	let parser2 = is(&2);
	let mut tokens = vec![3];
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut tokens), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut tokens).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_option_consuming_fails() {
	let is_1 = is(&1);
	let mut tokens = vec![1, 3];
	let consuming_parser = |input: &mut Vec<_>| {
		// Consume regardless of success.
		let next = input.next().unwrap(); // `next` consumes input.
		if next % 2 == 0 {
			is_1(input)
		} else {
			Err(Error::UnexpectedToken)
		}
	};
	let parse_option = option(&consuming_parser, &is_1);
	let output = parse_option(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
}

#[test]
fn parse_option_not_consuming_succeeds() {
	let is_1 = is(&1);
	let mut tokens = vec![1, 3];
	let non_consuming_parser = |input: &mut Vec<_>| {
		let next = input.next_as_ref().unwrap(); // `next_as_ref` does not consumes input.
		if next % 2 == 0 {
			input.next(); // Consume only if success.
			is_1(input)
		} else {
			Err(Error::UnexpectedToken)
		}
	};
	let parse_option = option(&non_consuming_parser, &is_1);
	let output = parse_option(&mut tokens);
	assert_eq!(output, Ok(1));
}