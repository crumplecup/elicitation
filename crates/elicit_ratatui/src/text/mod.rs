//! Text primitive shadow types: `Span`, `Line`, `Text`.

mod line;
mod span;
mod text_impl;

pub use line::Line;
pub use span::Span;
pub use text_impl::Text;
