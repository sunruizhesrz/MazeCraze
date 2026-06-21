use crate::core::{Cell, Grid, Point};

use super::Renderer;

/// ASCII renderer for maximum terminal compatibility.
///
/// Uses only basic ASCII characters: `+`, `-`, `|`, ` `.
pub struct AsciiRenderer;

impl AsciiRenderer {
    pub fn new() -> Self {
        Self
    }

    fn cell_char(&self, grid: &Grid, x: usize, y: usize) -> char {
        let p = Point::new(x, y);
        match grid.get(p) {
            Some(Cell::Wall) => {
                if x.is_multiple_of(2) {
                    '|'
                } else {
                    '-'
                }
            }
            Some(Cell::Passage) => ' ',
            Some(Cell::Visited) => '.',
            Some(Cell::Path) => '*',
            Some(Cell::Current) => '@',
            None => ' ',
        }
    }
}

impl Default for AsciiRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for AsciiRenderer {
    fn render(&self, grid: &Grid) -> String {
        let mut lines = Vec::with_capacity(grid.height());
        for y in 0..grid.height() {
            let mut line = String::with_capacity(grid.width());
            for x in 0..grid.width() {
                line.push(self.cell_char(grid, x, y));
            }
            lines.push(line);
        }
        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "ASCII"
    }
}
