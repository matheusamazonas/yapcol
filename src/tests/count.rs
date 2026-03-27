use crate::input::Position;
use crate::*;

#[test]
fn count_zero_empty() {
	let parser = is('h');
	let mut input = Input::new("".chars());
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn count_0_not_empty() {
	let parser = is('h');
	let mut input = Input::new("jello".chars());
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn count_all_same() {
	let parser = is('h');
	let repeat_count: usize = 500;
	let tokens: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count); // The count matched the request.
	assert!(output.iter().all(|x| *x == 'h')); // All values match the parser's.
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn count_one_different() {
	let parser = is('h');
	let repeat_count: usize = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
	tokens.push('x');
	let mut tail: Vec<_> = std::iter::repeat_n('h', repeat_count).collect();
	tokens.append(&mut tail);
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert!(output.iter().all(|x| *x == 'h')); // All values match the parser's.
	assert_eq!(input.consumed_count(), repeat_count); // Input was left intact.
	assert_eq!(any()(&mut input), Ok('x')); // Input was consumed as much as possible.
}

#[test]
fn count_not_enough() {
	let parser = is('h');
	let mut tokens: Vec<_> = std::iter::repeat_n('h', 3).collect();
	tokens.push('x');
	tokens.push('y');
	let mut input = Input::new(tokens);
	let parser = count(&parser, 4); // The 4th element is "other", so this should fail.
	let output = parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken(Position::new(1, 4))));
}
