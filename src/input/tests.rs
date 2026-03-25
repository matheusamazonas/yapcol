use crate::input::string::new_string_input;
use crate::input::PositionToken;

#[test]
fn lookahead_no_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler, false);
	assert!(input.look_ahead_buffer.is_empty());
	assert_eq!(input.consumed_count(), 2);
	assert_eq!(input.look_ahead_frames.len(), 0);
	assert_eq!(input.next_token(), None);
}

#[test]
fn lookahead_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler, true);
	assert!(!input.look_ahead_buffer.is_empty());
	assert_eq!(input.consumed_count(), 0);
	assert_eq!(input.next_token().unwrap().token(), '1');
}

#[test]
fn peek_twice() {
	let mut input = new_string_input("12".chars());
	assert_eq!(input.peek().unwrap().token(), '1');
	assert_eq!(input.peek().unwrap().token(), '1');
}

#[test]
fn peek_twice_while_looking_ahead_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.peek().unwrap().token(), '1');
	assert_eq!(input.peek().unwrap().token(), '1');
	input.stop_look_ahead(handler, true);
	assert_eq!(input.peek().unwrap().token(), '1');
}

#[test]
fn peek_twice_while_looking_ahead_not_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.peek().unwrap().token(), '1');
	assert_eq!(input.peek().unwrap().token(), '1');
	input.stop_look_ahead(handler, false);
	assert_eq!(input.peek().unwrap().token(), '1');
}

#[test]
fn repeat_peek_look_ahead_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	input.stop_look_ahead(handler, true);
	assert_eq!(input.peek().unwrap().token(), '1');

	let handler = input.start_look_ahead();
	assert_eq!(input.peek().unwrap().token(), '1');
	input.stop_look_ahead(handler, true);
	assert_eq!(input.peek().unwrap().token(), '1');
}

#[test]
fn repeat_peek_look_ahead_not_backtracking() {
	let mut input = new_string_input("12".chars());
	let handler = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	input.stop_look_ahead(handler, false);
	assert_eq!(input.peek().unwrap().token(), '2');

	let handler = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler, true);
	assert_eq!(input.peek().unwrap().token(), '2');
	assert_eq!(input.next_token().unwrap().token(), '2');
}

#[test]
fn nested_lookahead_backtrack() {
	let mut input = new_string_input("12345".chars());
	let handler1 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	let handler2 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler2, true);
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler1, true);
	assert_eq!(input.next_token().unwrap().token(), '1');
}

#[test]
fn nested_lookahead_no_backtrack() {
	let mut input = new_string_input("12345".chars());
	let handler1 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	let handler2 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler2, false);
	assert_eq!(input.next_token().unwrap().token(), '3');
	input.stop_look_ahead(handler1, false);
	assert_eq!(input.next_token().unwrap().token(), '4');
}

#[test]
fn nested_look_ahead_backtrack_first() {
	let mut input = new_string_input("12345".chars());
	let handler1 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	let handler2 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler2, true);
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler1, false);
	assert_eq!(input.next_token().unwrap().token(), '3');
}

#[test]
fn nested_look_ahead_backtrack_second() {
	let mut input = new_string_input("12345".chars());
	let handler1 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '1');
	let handler2 = input.start_look_ahead();
	assert_eq!(input.next_token().unwrap().token(), '2');
	input.stop_look_ahead(handler2, false);
	assert_eq!(input.next_token().unwrap().token(), '3');
	input.stop_look_ahead(handler1, true);
	assert_eq!(input.next_token().unwrap().token(), '1');
}

#[test]
#[should_panic]
fn wrong_token() {
	let mut input = new_string_input("12345".chars());
	let handler1 = input.start_look_ahead();
	let _handler2 = input.start_look_ahead();
	input.stop_look_ahead(handler1, false);
}
