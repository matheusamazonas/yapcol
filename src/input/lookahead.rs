/// A frame used to keep track of lookahead operations.
pub struct LookAheadFrame {
	/// Where in the lookahead buffer this frame started.
	pub start_index: usize,
	/// How many tokens this frame takes in the lookahead buffer.
	pub length: usize,
}

impl LookAheadFrame {
	/// The (lookahead buffer) index of the next token in this frame.
	pub fn next_ix(&self) -> usize {
		self.start_index + self.length
	}
}

/// A handler used to enforce the following on lookahead operations:
/// - That calls to [`Input::stop_look_ahead`] are only possible after a call to
///   [`Input::start_look_ahead`].
/// - That the right order of start/stop lookahead operations is performed.
#[must_use]
pub struct LookAheadHandler {
	id: usize,
}

impl LookAheadHandler {
	pub fn new(id: usize) -> Self {
		Self { id }
	}

	pub fn id(&self) -> usize {
		self.id
	}
}

/// Possible locations where the next input token might be.
pub enum TokenLocation {
	Stream,
	StreamLookingAhead,
	BufferHead,
	BufferTail,
}
