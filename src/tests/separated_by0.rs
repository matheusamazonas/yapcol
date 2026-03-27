use crate::input::Position;
use crate::*;

#[test]
fn empty() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn single_no_separator_succeeds() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec!['1']));
}

#[test]
fn single_dangling_separator_fails() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn two_with_separator_succeeds() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,1".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Ok(vec!['1', '1']));
}

#[test]
fn two_wrong_last_element_fails() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,2".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 3))));
}

#[test]
fn two_no_separator_succeeds() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("11".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output, vec!['1']);
}

#[test]
fn many_properly_separated_succeeds() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,1,1,1,1,1,1,1,1,1".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input).unwrap();
	assert_eq!(output.len(), 10);
}

#[test]
fn many_dangling_separator_fails() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,1,1,1,1,1,1,1,1,1,".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::EndOfInput));
}

#[test]
fn many_wrong_last_element_fails() {
	let parse_item = is('1');
	let parse_separator = is(',');
	let mut input = Input::new("1,1,1,1,1,1,1,1,1,1,2".chars());
	let output = separated_by0(&parse_item, &parse_separator)(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 21))));
}
