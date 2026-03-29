use crate::input::core::InputToken;

pub trait InputSource {
	type Token: InputToken;
	fn source_name(&self) -> String;
	fn next_token(&mut self) -> Option<Self::Token>;
	fn peek(&mut self) -> Option<&Self::Token>;
}
