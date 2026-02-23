use crate::*;

#[test]
fn parse_count_0_empty() {
	let input = String::from("hello");
	let parser = is(&input);
	let mut tokens: Vec<String> = vec![];
	let parser = count(&parser, 0);
	let output = parser(&mut tokens);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_0_not_empty() {
	let input = String::from("hello");
	let parser = is(&input);
	let mut tokens: Vec<String> = vec![String::from("other")];
	let parser = count(&parser, 0);
	let output = parser(&mut tokens);
	assert_eq!(output, Ok(vec![]));
}

#[test]
fn parse_count_all_same() {
	let input = String::from("hello");
	let parser = is(&input);
	let repeat_count: usize = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n(input.clone(), repeat_count).collect();
	let parser = count(&parser, repeat_count);
	let output = parser(&mut tokens).unwrap();
	assert_eq!(output.len(), repeat_count); // The count matched the request.
	assert!(output.iter().all(|x| *x == input));   // All values match the parser's.
	assert!(end_of_input()(&mut tokens).is_ok());  // Ensure that the input was consumed.
}

#[test]
fn parse_count_one_different() {
	let input = String::from("hello");
	let parser = is(&input);
	let repeat_count: usize = 500;
	let mut tokens: Vec<_> = std::iter::repeat_n(input.clone(), repeat_count).collect();
	let other = String::from("other");
	tokens.push(other.clone());
	let mut tail = std::iter::repeat_n(input.clone(), repeat_count).collect();
	tokens.append(&mut tail);
	let parser = count(&parser, repeat_count);
	let output = parser(&mut tokens).unwrap();
	assert!(output.iter().all(|x| *x == input));     // All values match the parser's.
	assert_eq!(tokens.remove(0), other);             // Input was consumed as much as possible.
	assert_eq!(tokens.len(), repeat_count); // Input was left intact.
}

#[test]
fn parse_count_not_enough() {
	let input = String::from("hello");
	let other = String::from("other");
	let parser = is(&input);
	let mut tokens: Vec<_> = std::iter::repeat_n(input.clone(), 3).collect();
	tokens.push(other.clone());
	tokens.push(input.clone());
	let parser = count(&parser, 4); // The 4th element is "other", so this should fail.
	let output = parser(&mut tokens);
	assert_eq!(output, Err(Error::UnexpectedToken));
}
