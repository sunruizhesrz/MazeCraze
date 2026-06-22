//! 迷宫导出功能，将迷宫保存为外部格式。

pub mod text;

pub use text::export_to_text;

use crate::core::Grid;
use std::error::Error;
use std::path::Path;

/// 将网格导出到指定路径的文件。
/// 文件格式由文件扩展名决定。
pub fn export_grid(grid: &Grid, path: &Path) -> Result<(), Box<dyn Error>> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");
    match ext {
        "txt" => export_to_text(grid, path),
        _ => Err(format!("Unsupported export format: {}", ext).into()),
    }
}
