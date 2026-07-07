use crate::input::Position;
use crate::{Error, Input, InputToken, Mismatch, Parser};
use std::marker::PhantomData;

pub trait RepetitionAccumulator<I, O> {
	fn new() -> Self;
	fn add(&mut self, value: I);
	fn value(self) -> O;
}

pub struct MatchesAccumulator<T> {
	matches: Vec<T>,
}

impl<T> RepetitionAccumulator<T, Vec<T>> for MatchesAccumulator<T> {
	fn new() -> Self {
		MatchesAccumulator {
			matches: Vec::new(),
		}
	}

	fn add(&mut self, value: T) {
		self.matches.push(value);
	}

	fn value(self) -> Vec<T> {
		self.matches
	}
}

pub struct CountAccumulator<T> {
	count: usize,
	phantom: PhantomData<T>,
}

impl<T> RepetitionAccumulator<T, usize> for CountAccumulator<T> {
	fn new() -> Self {
		CountAccumulator {
			count: 0,
			phantom: PhantomData,
		}
	}

	fn add(&mut self, _: T) {
		self.count += 1;
	}

	fn value(self) -> usize {
		self.count
	}
}

fn fail<IT>(_: &mut Input<IT>) -> Result<(), Error> {
	Err(Error::UnexpectedToken(None, Position::new(0, 0), None))
}

pub fn repeat_no_end<P, IT, O, A, AO>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
) -> impl Parser<IT, A>
where
	P: Parser<IT, O>,
	IT: InputToken,
	A: RepetitionAccumulator<O, AO>,
{
	move |input| repeat(parser, min_match_count, max_match_count, false, &fail)(input)
}

pub fn repeat_with_end<P, PE, IT, O, OE, A, AO>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
	end_parser: &PE,
) -> impl Parser<IT, A>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
	A: RepetitionAccumulator<O, AO>,
{
	repeat(parser, min_match_count, max_match_count, true, end_parser)
}

fn repeat<P, PS, IT, O, OP, A, AO>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
	fail_on_error: bool,
	end_parser: &PS,
) -> impl Parser<IT, A>
where
	P: Parser<IT, O>,
	PS: Parser<IT, OP>,
	IT: InputToken,
	A: RepetitionAccumulator<O, AO>,
{
	move |input| {
		let mut accumulator = A::new();
		let mut total_match_count = 0;
		let mut previous_consumed_count = input.consumed_count();
		while end_parser(input).is_err() {
			let previous_position = input.position();
			let outcome = parser(input);
			match (outcome, max_match_count) {
				// Matched too many times.
				(Ok(_), Some(max_count)) if max_count == total_match_count => {
					total_match_count += 1;
					let expected = format!("at most {max_count} occurrences");
					let found = format!("{total_match_count} occurrences");
					let mismatch = Mismatch::new(expected, found);
					return Err(Error::UnexpectedToken(
						input.source_name(),
						previous_position,
						Some(mismatch),
					));
				}
				// Valid match.
				(Ok(token), _) => {
					total_match_count += 1;
					let consumed_count = input.consumed_count();
					// Check if non-consuming parser. If so, it would cause an infinite loop.
					if previous_consumed_count == consumed_count {
						return Err(Error::NonConsumingLoop(
							input.source_name(),
							input.position(),
						));
					}
					accumulator.add(token);
					previous_consumed_count = consumed_count;
				}
				(Err(e), _) if fail_on_error => return Err(e),
				(Err(_), _) if total_match_count >= min_match_count => return Ok(accumulator),
				(Err(e), _) => return Err(e),
			}
		}
		if total_match_count >= min_match_count {
			Ok(accumulator)
		} else {
			let expected = format!("at least {min_match_count} occurrences");
			let found = format!("{total_match_count} occurrences");
			let mismatch = Mismatch::new(expected, found);
			Err(Error::UnexpectedToken(
				input.source_name(),
				input.position(),
				Some(mismatch),
			))
		}
	}
}

#[cfg(test)]
mod count_accumulator_tests {
	use super::*;

	#[test]
	fn new_value_is_zero() {
		let accumulator = CountAccumulator::<()>::new();
		let count = accumulator.value();
		assert_eq!(count, 0);
	}

	#[test]
	fn add_once_is_one() {
		let mut accumulator = CountAccumulator::<()>::new();
		accumulator.add(());
		let count = accumulator.value();
		assert_eq!(count, 1);
	}
}

#[cfg(test)]
mod matches_accumulator_tests {
	use super::*;

	#[test]
	fn new_value_is_zero() {
		let accumulator = MatchesAccumulator::<()>::new();
		let count = accumulator.value();
		assert_eq!(count, vec![]);
	}

	#[test]
	fn add_once_has_item() {
		let mut accumulator = MatchesAccumulator::<usize>::new();
		accumulator.add(42);
		let count = accumulator.value();
		assert_eq!(count, vec![42]);
	}
}
