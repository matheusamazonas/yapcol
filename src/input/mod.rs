//! This module provides the [`Input`] type, which wraps any iterator and exposes it as a token
//! stream that parsers can consume. It is the primary interface through which parsers read tokens,
//! and it is the only type in this module that users need to interact with directly.
//!
//! # Tokens
//!
//! Any type that implements [`PartialEq`] and [`Clone`] can be used as a token. This is
//! automatically satisfied via the blanket implementation of the [`Token`] trait, so no manual
//! implementation is required.
//!
//! # Creating an input stream
//!
//! An [`Input`] is constructed from any type that implements [`IntoIterator`], where the iterator
//! elements must implement [`Token`]:
//!
//! ```
//! use yapcol::input::core::Input;
//!
//! let tokens = "hello".chars();
//! let mut input = Input::new_from_chars(tokens);
//! ```
//!
//! # Lookahead
//!
//! [`Input`] supports arbitrary lookahead: tokens can be fetched and inspected without being
//! permanently consumed. A lookahead operation is started with
//! [`Input::start_look_ahead`](Input::start_look_ahead) and stopped with
//! [`Input::stop_look_ahead`](Input::stop_look_ahead). When stopping, the caller decides whether
//! to backtrack (keeping the fetched tokens available for re-reading) or to discard them.
//!
//! Lookahead operations can be nested. Each nested operation is self-contained, and they must be
//! stopped in the reverse order of their creation (last started, first stopped).
//!
//! In practice, lookahead is used internally by parser combinators such as [`crate::attempt`] and
//! [`crate::look_ahead`] defined in the crate root, so most users will not need to call these
//! methods directly.

pub mod core;
mod lookahead;
pub mod position;
mod source;
pub mod string;
mod token;
