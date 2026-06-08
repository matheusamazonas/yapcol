use crate::{Error, InputToken, Mismatch, Parser};
use ManyOutput::{Count, Matches};

pub enum ManyOutput<T> {
	Matches(Vec<T>),
	Count(usize),
}

pub fn many<P, IT, O>(
	parser: &P,
	min_match_count: usize,
	max_match_count: Option<usize>,
	store_matches: bool,
) -> impl Parser<IT, ManyOutput<O>>
where
	P: Parser<IT, O>,
	IT: InputToken,
{
	move |input| {
		let mut output = if store_matches {
			Matches(Vec::new())
		} else {
			Count(0)
		};
		let mut total_match_count = 0;
		let mut previous_consumed_count = input.consumed_count();
		loop {
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
					match output {
						Matches(ref mut matches) => matches.push(token),
						Count(ref mut count) => *count += 1,
					}
					previous_consumed_count = consumed_count;
				}
				(Err(e), _) => {
					return if total_match_count >= min_match_count {
						Ok(output)
					} else {
						Err(e)
					};
				}
			}
		}
	}
}
