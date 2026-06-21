use crate::core::{Cell, Grid, Point};

use super::Renderer;

/// Unicode Box Drawing renderer.
///
/// Produces aesthetically pleasing output using Unicode box-drawing characters
/// and ANSI color codes.
pub struct UnicodeRenderer {
    use_color: bool,
}

impl UnicodeRenderer {
    pub fn new() -> Self {
        Self { use_color: true }
    }

    pub fn with_color(mut self, enabled: bool) -> Self {
        self.use_color = enabled;
        self
    }

    fn cell_char(&self, grid: &Grid, x: usize, y: usize) -> String {
        let p = Point::new(x, y);
        let cell = grid.get(p);

        if x.is_multiple_of(2) && y.is_multiple_of(2) {
            // Intersection
            return self.styled("┼", "intersection");
        }

        match cell {
            Some(Cell::Wall) => {
                if x.is_multiple_of(2) {
                    self.styled("│", "wall_v")
                } else {
                    self.styled("─", "wall_h")
                }
            }
            Some(Cell::Passage) => self.styled(" ", "passage"),
            Some(Cell::Visited) => self.styled("·", "visited"),
            Some(Cell::Path) => self.styled("●", "path"),
            Some(Cell::Current) => self.styled("◆", "current"),
            None => " ".to_string(),
        }
    }

    fn styled(&self, ch: &str, style: &str) -> String {
        if !self.use_color {
            return ch.to_string();
        }
        match style {
            "wall_h" | "wall_v" | "intersection" => format!("\x1b[38;5;240m{}\x1b[0m", ch),
            "passage" => ch.to_string(),
            "visited" => format!("\x1b[38;5;33m{}\x1b[0m", ch),
            "path" => format!("\x1b[38;5;82m{}\x1b[0m", ch),
            "current" => format!("\x1b[38;5;226m{}\x1b[0m", ch),
            _ => ch.to_string(),
        }
    }
}

impl Default for UnicodeRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer for UnicodeRenderer {
    fn render(&self, grid: &Grid) -> String {
        let mut lines = Vec::with_capacity(grid.height());
        for y in 0..grid.height() {
            let mut line = String::with_capacity(grid.width());
            for x in 0..grid.width() {
                line.push_str(&self.cell_char(grid, x, y));
            }
            lines.push(line);
        }
        lines.join("\n")
    }

    fn name(&self) -> &'static str {
        "Unicode"
    }
}
