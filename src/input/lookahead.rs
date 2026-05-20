/// A frame used to keep track of lookahead operations.
pub struct LookAheadFrame {
	/// Where in the lookahead buffer this frame started.
	start_index: usize,
	/// How many tokens this frame takes in the lookahead buffer.
	length: usize,
}

impl LookAheadFrame {
	/// Creates a new [`LookAheadFrame`] starting at `start_index` in the lookahead buffer.
	pub fn new(start_index: usize) -> LookAheadFrame {
		LookAheadFrame {
			start_index,
			length: 0,
		}
	}

	/// Returns the number of tokens in this frame.
	pub fn length(&self) -> usize {
		self.length
	}

	/// The lookahead buffer index of the token right after this frame.
	pub fn next_token_ix(&self) -> usize {
		self.start_index + self.length
	}

	/// Increments the number of tokens placed in this frame.
	pub fn increment(&mut self) {
		self.length += 1;
	}
}

/// A handler used to enforce the following on lookahead operations:
/// - That calls to [`Input::stop_look_ahead`] are only possible after a call to
///   [`Input::start_look_ahead`].
/// - That the right order of start/stop lookahead operations is performed.
#[must_use]
pub(crate) struct LookAheadHandler {
	id: usize,
}

impl LookAheadHandler {
	/// Creates a new [`LookAheadHandler`] with the given `id`.
	pub fn new(id: usize) -> Self {
		Self { id }
	}

	/// Returns the unique identifier of this handler.
	pub fn id(&self) -> usize {
		self.id
	}
}

/// Possible locations where the next input token might be.
pub enum TokenLocation {
	/// The next token will be fetched directly from the underlying source stream.
	Stream,
	/// The input is in lookahead mode, and the next token will be fetched from the source
	/// stream and pushed onto the lookahead buffer.
	StreamLookingAhead,
	/// The next token will be taken from the front of the lookahead buffer (normal
	/// consumption after a backtracking lookahead).
	BufferHead,
	/// The input is in lookahead mode, and the next token will be read from an existing
	/// position inside the lookahead buffer.
	BufferTail,
}
