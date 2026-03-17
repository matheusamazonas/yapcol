use crate::*;

#[test]
fn success_first() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let tokens = vec!["hello"];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, "hello");
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn success_second() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let tokens = vec!["hallo"];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	let output = parse_option(&mut input).unwrap();
	assert_eq!(output, "hallo");
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn fail_not_consuming() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let tokens = vec!["other"];
	let mut input = Input::new(tokens);
	let parse_option = option(&parser1, &parser2);
	assert_eq!(parse_option(&mut input), Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn fail_consuming() {
	let parser1 = is("hello");
	let parser2 = is("hallo");
	let tokens = vec!["hello", "hello"];
	let mut input = Input::new(tokens);
	let consuming_parser = |input: &mut Input<_>| {
		parser1(input)?;
		parser2(input)
	};
	let parse_option = option(&consuming_parser, &parser2);
	let output = parse_option(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
