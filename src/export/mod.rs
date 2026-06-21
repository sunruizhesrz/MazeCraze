//! Export functionality for saving mazes to external formats.

pub mod text;

pub use text::export_to_text;

use crate::core::Grid;
use std::error::Error;
use std::path::Path;

/// Export a grid to a file at the given path.
/// The format is determined by the file extension.
pub fn export_grid(grid: &Grid, path: &Path) -> Result<(), Box<dyn Error>> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
    match ext {
        "txt" => export_to_text(grid, path),
        _ => Err(format!("Unsupported export format: {}", ext).into()),
    }
}
