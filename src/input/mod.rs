//! This module provides the [`core::Input`] type and the [`core::InputToken`] trait — the two
//! main building blocks for feeding data into parsers.
//!
//! [`core::Input`] wraps any iterator and exposes it as a token stream that parsers can consume.
//! It is the primary interface through which parsers read tokens.
//!
//! [`core::InputToken`] describes a single element in that stream. For character-based parsing,
//! there is no need to implement the trait yourself: the library supplies an implementation via
//! [`string::CharToken`].
//! You can also define streams of custom tokens (e.g., from a lexer) by implementing
//! [`core::InputToken`] on the custom token type.
//!
//! # Tokens
//!
//! A token is any type that implements [`core::InputToken`]. The trait requires [`Clone`] and
//! an associated `Token` type that implements [`PartialEq`] and [`Clone`].
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
//! use yapcol::input::core::Input;
//!
//! let mut input = Input::new_from_chars("hello".chars(), None);
//! ```
//!
//! ## Token-based parsing
//!
//! When you have performed lexical analysis beforehand and want to parse a stream of structured
//! tokens, implement [`core::InputToken`] on your token type and use
//! [`core::Input::new_from_tokens`]:
//!
//! ```ignore
//! use yapcol::input::core::Input;
//!
//! let tokens: Vec<MyToken> = lexer.tokenize(source);
//! let mut input = Input::new_from_tokens(tokens, None);
//! ```
//!
//! # Position tracking
//!
//! Every token carries a [`position::Position`] (line and column, both 1-based) that records
//! where in the source it appeared. [`core::Input::position`] returns the position of the next
//! token to be consumed, or the position of the last consumed token when the stream is
//! exhausted. Positions are included in errors to produce precise error messages.
//!
//! # Submodules
//!
//! - [`core`]: The [`core::Input`] stream and the [`core::InputToken`] trait.
//! - [`position`]: The [`position::Position`] type used for source location tracking.
//! - [`string`]: Character-stream input via [`string::StringInput`] and [`string::CharToken`].

pub mod core;
mod lookahead;
pub mod position;
mod source;
pub mod string;
mod token;
