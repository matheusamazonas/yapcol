//! This module provides the [`Input`] type and the [`InputToken`] trait — the two main building
//! blocks for feeding data into parsers.
//!
//! [`Input`] wraps any iterator and exposes it as a token stream that parsers can consume.
//! It is the primary interface through which parsers read tokens.
//!
//! [`InputToken`] describes a single element in that stream. For character-based parsing,
//! there is no need to implement the trait yourself: the library supplies an implementation via
//! [`CharToken`].
//! You can also define streams of custom tokens (e.g., from a lexer) by implementing
//! [`InputToken`] on the custom token type.
//!
//! # Tokens
//!
//! A token is any type that implements [`InputToken`]. The trait requires [`Clone`] and an
//! associated `Token` type that implements [`PartialEq`] and [`Clone`].
//!
//! # Creating an input stream
//!
//! There are two main ways to create an input stream, depending on whether you are parsing raw
//! characters or structured tokens:
//!
//! ## Character-based parsing
//!
//! Use [`core::Input::new_from_chars`], which wraps a `char` iterator and automatically
//! tracks source positions:
//!
//! ```
//! use yapcol::Input;
//!
//! let mut input = Input::new_from_chars("hello".chars(), None);
//! ```
//!
//! ## Token-based parsing
//!
//! When you have performed lexical analysis beforehand and want to parse a stream of structured
//! tokens, implement [`InputToken`] on your token type and use [`Input::new_from_tokens`]:
//!
//! ```rust,ignore
//! use yapcol::input::core::Input;
//!
//! let tokens: Vec<MyToken> = lexer.tokenize(source);
//! let mut input = Input::new_from_tokens(tokens, None);
//! ```
//!
//! # Position tracking
//!
//! Every token carries a [`Position`] (line and column, both 1-based) that records where in the
//! source it appeared. [`Input::position`] returns the position of the next token to be consumed,
//! or the position of the last consumed token when the stream is exhausted. Positions are included
//! in errors to produce precise error messages.

mod core;
mod lookahead;
mod position;
mod source;
mod string;
mod token;

pub use core::{Input, InputToken};
pub use position::Position;
pub use string::{CharToken, StringInput};
