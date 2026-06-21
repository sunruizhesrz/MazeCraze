//! Rendering engines for maze visualization.
//!
//! Each renderer implements the [`Renderer`] trait and converts a [`Grid`]
//! into a human-readable string representation.

pub mod ascii;
pub mod unicode;

pub use ascii::AsciiRenderer;
pub use unicode::UnicodeRenderer;

/// A renderer converts a [`Grid`] into a string representation.
pub trait Renderer {
    /// Render the grid as a string.
    fn render(&self, grid: &crate::core::Grid) -> String;

    /// Human-readable name.
    fn name(&self) -> &'static str;
}

/// Registry of available renderers.
pub fn all_renderers() -> Vec<Box<dyn Renderer>> {
    vec![
        Box::new(UnicodeRenderer::new()),
        Box::new(AsciiRenderer::new()),
    ]
}
