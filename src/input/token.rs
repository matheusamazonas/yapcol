/// The smallest unit of input supported. If you'd like to use a custom type as tokens (e.g.,
/// for lexical analysis, a.k.a. lexing), implementing [`PartialEq`] and [`Clone`] is enough to
/// make it token-compatible.
pub trait Token: PartialEq + Clone {}

impl<T> Token for T where T: PartialEq + Clone {}