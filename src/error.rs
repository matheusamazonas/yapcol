#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum Error {
	UnexpectedToken,
	EndOfInput
}