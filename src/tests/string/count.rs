use crate::*;

#[test]
fn parse_count_0_empty() {
	let token = String::from("hello");
	let parser = is(&token);
	let tokens: Vec<String> = vec![];
	let mut input = Input::new(tokens);
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_0_not_empty() {
	let input = String::from("hello");
	let parser = is(&input);
	let tokens: Vec<String> = vec![String::from("other")];
	let mut input = Input::new(tokens);
	let parser = count(&parser, 0);
	let output = parser(&mut input);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_all_same() {
	let token = String::from("hello");
	let parser = is(&token);
	let repeat_count: usize = 500;
	let tokens: Vec<_> = std::iter::repeat_n(token.clone(), repeat_count).collect();
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert_eq!(output.len(), repeat_count); // The count matched the request.
	assert!(output.iter().all(|x| *x == token)); // All values match the parser's.
	assert!(end_of_input()(&mut input).is_ok()); // Ensure that the input was consumed.
}

#[test]
fn parse_count_one_different() {
	let token = String::from("hello");
	let parser = is(&token);
	let repeat_count: usize = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n(token.clone(), repeat_count).collect();
	let other = String::from("other");
	tokens.push(other.clone());
	let mut tail = std::iter::repeat_n(token.clone(), repeat_count).collect();
	tokens.append(&mut tail);
	let mut input = Input::new(tokens);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut input).unwrap();
	assert!(output.iter().all(|x| *x == token)); // All values match the parser's.
	assert_eq!(input.consumed_count(), repeat_count as u32); // Input was left intact.
	assert_eq!(input.next_token(), Some(other)); // Input was consumed as much as possible.
}

#[test]
fn parse_count_not_enough() {
	let token = String::from("hello");
	let other = String::from("other");
	let parser = is(&token);
	let mut tokens: Vec<_> = std::iter::repeat_n(token.clone(), 3).collect();
	tokens.push(other.clone());
	tokens.push(token.clone());
	let mut input = Input::new(tokens);
	let parser = count(&parser, 4); // The 4th element is "other", so this should fail.
	let output = parser(&mut input);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
