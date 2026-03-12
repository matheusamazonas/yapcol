use crate::*;

#[test]
fn parse_count_0_empty() {
	let parser = is(1);
	let tokens: Vec<i32> = vec![];
	let mut input = Input::new(tokens);
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_0_not_empty() {
	let parser = is(1);
	let tokens: Vec<i32> = vec![4];
	let mut input = Input::new(tokens);
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_all_same() {
	let parser = is(1);
	let repeat_count: usize = 500;
	let tokens: Vec<i32> = std::iter::repeat_n(1, repeat_count).collect();
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count); // The count matched the request.
	assert!(output.iter().all(|&x| x == 1)); // All values match the parser's.
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_count_one_different() {
	let parser = is(1);
	let repeat_count: usize = 500;
	let mut tokens: Vec<i32> = std::iter::repeat_n(1, repeat_count).collect();
	tokens.push(42);
	let mut tail = std::iter::repeat_n(1, repeat_count).collect();
	tokens.append(&mut tail);
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert!(output.iter().all(|&x| x == 1)); // All values match the parser's.
	assert_eq!(input.next_token(), Some(42)); // Input was consumed as much as possible.
	assert_eq!(input.consumed_count(), repeat_count + 1); // Input was left intact.
}

#[test]
fn parse_count_not_enough() {
	let parser = is(1);
	let tokens = vec![1, 1, 1, 2, 1];
	let mut input = Input::new(tokens);
	let parser = count(&parser, 5);
	let output = parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
