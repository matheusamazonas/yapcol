use crate::*;

#[test]
fn parse_empty() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut input = Vec::new();
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
}

#[test]
fn parse_wrong_does_not_consume_input() {
	let hello = String::from("hello");
	let hallo = String::from("hallo");
	let parser = is(&hello);
	let mut input = vec![hallo.clone()];
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Err(Error::UnexpectedToken));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}

#[test]
fn parse_does_not_consume_on_success() {
	let hello = String::from("hello");
	let world = String::from("world");
	let foo = String::from("foo");
	let parser = is(&hello);
	let mut input = vec![hello.clone(), world.clone(), foo.clone()];
	let output = look_ahead(&parser)(&mut input);
	assert_eq!(output, Ok(hello.clone()));
	// After look_ahead, input should still start with hello
	assert_eq!(is(&hello)(&mut input), Ok(hello.clone()));
	assert_eq!(is(&world)(&mut input), Ok(world.clone()));
	assert_eq!(is(&foo)(&mut input), Ok(foo.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_does_not_consume_on_failure() {
	let hello = String::from("hello");
	let world = String::from("world");
	let foo = String::from("foo");
	let parser = is(&hello);
	let mut input = vec![world.clone(), foo.clone()];
	let result = look_ahead(&parser)(&mut input);
	assert_eq!(result, Err(Error::UnexpectedToken));
	// Input should still be intact
	assert_eq!(is(&world)(&mut input), Ok(world.clone()));
	assert_eq!(is(&foo)(&mut input), Ok(foo.clone()));
	assert!(end_of_input()(&mut input).is_ok());
}

#[test]
fn parse_right_multiple_elements() {
	let hello = String::from("hello");
	let world = String::from("world");
	let parser = is(&hello);
	let mut input = vec![hello.clone(), world.clone()];
	let parse_look_ahead = look_ahead(&parser)(&mut input);
	assert_eq!(parse_look_ahead, Ok(hello.clone()));
	assert_eq!(input, vec![hello.clone(), world.clone()]); // Ensure the input was NOT consumed.
}

#[test]
fn parse_look_ahead_twice() {
	let hello = String::from("hello");
	let parser = is(&hello);
	let mut input = vec![hello.clone()];
	let first = look_ahead(&parser)(&mut input);
	let second = look_ahead(&parser)(&mut input);
	assert_eq!(first, Ok(hello.clone()));
	assert_eq!(second, Ok(hello.clone()));
	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
}