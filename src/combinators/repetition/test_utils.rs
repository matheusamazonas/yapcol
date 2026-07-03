use crate::Error;
use crate::input::Position;
use std::fmt::Debug;

pub fn assert_unexpected_error<T>(
	value: Result<T, Error>,
	position: Position,
	expected: &str,
	found: &str,
) where
	T: Debug,
{
	let error = value.unwrap_err();
	if let Error::UnexpectedToken(_, error_pos, mismatch) = error {
		if error_pos != position {
			panic!("Expected error position to be {position}, but got {error_pos}");
		}
		let mismatch_message = mismatch.unwrap().to_string();
		let mut split = mismatch_message.split("found:");
		let expected_message = split.next().unwrap();
		assert!(expected_message.contains(expected));
		let found_message = split.next().unwrap();
		assert!(found_message.contains(found));
	} else {
		panic!(
			"Expected error to be of type UnexpectedToken, but got {:?}",
			error
		);
	}
}
