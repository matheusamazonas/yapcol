use crate::input::Position;
use crate::{Error, Input, InputToken, Mismatch, Parser};
use std::marker::PhantomData;

pub trait RepetitionAccumulator<I, O, E> {
	fn new() -> Self;
	fn add(&mut self, value: I);
	fn end(&mut self, value: Option<E>);
	fn result(self) -> (O, Option<E>);
}

pub struct MatchesAccumulator<T, E> {
	matches: Vec<T>,
	end: Option<E>,
}

impl<T, E> RepetitionAccumulator<T, Vec<T>, E> for MatchesAccumulator<T, E> {
	fn new() -> Self {
		MatchesAccumulator {
			matches: Vec::new(),
			end: None,
		}
	}

	fn add(&mut self, value: T) {
		if self.end.is_some() {
			panic!("Cannot add more matches after the end of the repetition.");
		}
		self.matches.push(value);
	}

	fn end(&mut self, value: Option<E>) {
		self.end = value;
	}

	fn result(self) -> (Vec<T>, Option<E>) {
		(self.matches, self.end)
	}
}

pub struct CountAccumulator<T, E> {
	count: usize,
	phantom: PhantomData<T>,
	end: Option<E>,
}

impl<T, E> RepetitionAccumulator<T, usize, E> for CountAccumulator<T, E> {
	fn new() -> Self {
		CountAccumulator {
			count: 0,
			phantom: PhantomData,
			end: None,
		}
	}

	fn add(&mut self, _: T) {
		if self.end.is_some() {
			panic!("Cannot add more matches after the end of the repetition.");
		}
		self.count += 1;
	}

	fn end(&mut self, end: Option<E>) {
		// The end if not used, but it's useful to detect wrong calls to `add`.
		self.end = end
	}

	fn result(self) -> (usize, Option<E>) {
		(self.count, None)
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
	A: RepetitionAccumulator<O, AO, ()>,
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
	A: RepetitionAccumulator<O, AO, OE>,
{
	repeat(parser, min_match_count, max_match_count, true, end_parser)
}

fn repeat<P, PE, IT, O, OE, A, AO>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
	fail_on_error: bool,
	end_parser: &PE,
) -> impl Parser<IT, A>
where
	P: Parser<IT, O>,
	PE: Parser<IT, OE>,
	IT: InputToken,
	A: RepetitionAccumulator<O, AO, OE>,
{
	move |input| {
		let mut accumulator = A::new();
		let mut total_match_count = 0;
		let mut previous_consumed_count = input.consumed_count();
		let mut end_output = end_parser(input);
		while end_output.is_err() {
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
			end_output = end_parser(input);
		}
		if total_match_count >= min_match_count {
			accumulator.end(end_output.ok());
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
		let accumulator = CountAccumulator::<(), ()>::new();
		let result = accumulator.result();
		assert_eq!(result, (0, None));
	}

	#[test]
	fn add_once_is_one() {
		let mut accumulator = CountAccumulator::<(), ()>::new();
		accumulator.add(());
		let result = accumulator.result();
		assert_eq!(result, (1, None));
	}
}

#[cfg(test)]
mod matches_accumulator_tests {
	use super::*;

	#[test]
	fn new_value_is_zero() {
		let accumulator = MatchesAccumulator::<(), ()>::new();
		let result = accumulator.result();
		assert_eq!(result, (vec![], None));
	}

	#[test]
	fn add_once_has_item() {
		let mut accumulator = MatchesAccumulator::<usize, ()>::new();
		accumulator.add(42);
		let result = accumulator.result();
		assert_eq!(result, (vec![42], None));
	}

	#[test]
	fn end_match() {
		let mut accumulator = MatchesAccumulator::<usize, bool>::new();
		accumulator.add(42);
		accumulator.end(Some(true));
		let result = accumulator.result();
		assert_eq!(result, (vec![42], Some(true)));
	}
}
