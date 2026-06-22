use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::core::Grid;
use crate::renderer::{Renderer, UnicodeRenderer};

/// 使用 Unicode 渲染器将迷宫网格导出为纯文本文件。
pub fn export_to_text(grid: &Grid, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let renderer = UnicodeRenderer::new().with_color(false);
    let rendered = renderer.render(grid);

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", rendered)?;
    writer.flush()?;

    Ok(())
}
