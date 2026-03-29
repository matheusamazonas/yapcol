//! This module provides the [`core::Input`] type, which wraps any iterator and exposes it as a token
//! stream that parsers can consume. It is the primary interface through which parsers read tokens,
//! and it is the only type in this module that users need to interact with directly.
//!
//! # Tokens
//!
//! Any type that implements [`PartialEq`] and [`Clone`] can be used as a token. This is
//! automatically satisfied via the blanket implementation of the [`core::InputToken`] trait,
//! so no manual implementation is required.
//!
//! # Creating an input stream
//!
//! An [`core::Input`] is constructed from any type that implements [`IntoIterator`], where the
//! iterator elements must implement [`core::InputToken`]:
//!
//! ```
//! use yapcol::input::core::Input;
//!
//! let tokens = "hello".chars();
//! let mut input = Input::new_from_chars(tokens, None);
//! ```

pub mod core;
mod lookahead;
pub mod position;
pub mod source;
pub mod string;
mod token;
