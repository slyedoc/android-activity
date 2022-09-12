/// This struct holds a span within a region of text from `start` (inclusive) to
/// `end` (exclusive).
///
/// An empty span or cursor position is specified with `Some(start) == Some(end)`.
///
/// An undefined span is specified with start = end = `None`.
#[derive(Debug, Clone, Copy)]
pub struct TextSpan {
    /// The start of the span (inclusive)
    pub start: Option<usize>,

    /// The end of the span (exclusive)
    pub end: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct TextInputState {
    pub text: String,
    /// A selection defined on the text.
    pub selection: TextSpan,
    /// A composing region defined on the text.
    pub compose_region: TextSpan,
}

pub use crate::activity_impl::input::*;
