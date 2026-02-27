// use crate::*;
// 
// #[test]
// fn parse_empty() {
// 	let mut input: Vec<i32> = Vec::new();
// 	let parser = is(&(1));
// 	let parse_look_ahead = look_ahead(&parser)(&mut input);
// 	assert_eq!(parse_look_ahead, Err(Error::EndOfInput));
// }
// 
// #[test]
// fn parse_wrong_does_not_consume_input() {
// 	let mut input = vec![2];
// 	let parser = is(&(1));
// 	let parse_look_ahead = look_ahead(&parser)(&mut input);
// 	assert_eq!(parse_look_ahead, Err(Error::UnexpectedToken));
// 	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
// }
// 
// #[test]
// fn parse_does_not_consume_on_success() {
// 	let mut input = vec![1, 2, 3];
// 	let parser = is(&(1));
// 	let output = look_ahead(&parser)(&mut input);
// 	assert_eq!(output, Ok(1));
// 	// After look_ahead, input should still start with 1
// 	assert_eq!(is(&(1))(&mut input), Ok(1));
// 	assert_eq!(is(&(2))(&mut input), Ok(2));
// 	assert_eq!(is(&(3))(&mut input), Ok(3));
// 	assert!(end_of_input()(&mut input).is_ok());
// }
// 
// #[test]
// fn parse_does_not_consume_on_failure() {
// 	let mut input = vec![2, 3];
// 	let parser = is(&(1));
// 	let result = look_ahead(&parser)(&mut input);
// 	assert_eq!(result, Err(Error::UnexpectedToken));
// 	// Input should still be intact
// 	assert_eq!(is(&(2))(&mut input), Ok(2));
// 	assert_eq!(is(&(3))(&mut input), Ok(3));
// 	assert!(end_of_input()(&mut input).is_ok());
// }
// 
// #[test]
// fn parse_right_multiple_elements() {
// 	let mut input = vec![1, 2, 3];
// 	let parser = is(&(1));
// 	let parse_look_ahead = look_ahead(&parser)(&mut input);
// 	assert_eq!(parse_look_ahead, Ok(1));
// 	assert_eq!(input, vec![1, 2, 3]);
// }
// 
// #[test]
// fn parse_look_ahead_twice() {
// 	let mut input = vec![1];
// 	let parser = is(&(1));
// 	let first = look_ahead(&parser)(&mut input);
// 	let second = look_ahead(&parser)(&mut input);
// 	assert_eq!(first, Ok(1));
// 	assert_eq!(second, Ok(1));
// 	assert!(end_of_input()(&mut input).is_err()); // Ensure that the input was NOT consumed.
// }