use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::Widget,
};

use crate::core::{Cell, Grid, Point};

/// 将迷宫网格直接渲染到 ratatui 缓冲区中的组件。
pub struct MazeCanvas<'a> {
    grid: &'a Grid,
}

impl<'a> MazeCanvas<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self { grid }
    }

    fn cell_style(&self, cell: Cell) -> (char, Style) {
        match cell {
            Cell::Wall => ('█', Style::default().fg(Color::Black)),
            Cell::Passage => (' ', Style::default().bg(Color::White)),
            Cell::Visited => ('░', Style::default().fg(Color::Rgb(255, 140, 0))),
            Cell::Path => ('★', Style::default().fg(Color::Rgb(0, 255, 0))),
            Cell::Current => ('◆', Style::default().fg(Color::Rgb(255, 0, 255))),
        }
    }
}

impl<'a> Widget for MazeCanvas<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gw = self.grid.width() as u16;
        let gh = self.grid.height() as u16;

        // 在可用区域内居中迷宫
        let offset_x = area.x.saturating_add(area.width.saturating_sub(gw) / 2);
        let offset_y = area.y.saturating_add(area.height.saturating_sub(gh) / 2);

        for y in 0..gh.min(area.height) {
            for x in 0..gw.min(area.width) {
                if let Some(cell) = self.grid.get(Point::new(x as usize, y as usize)) {
                    let (ch, style) = self.cell_style(*cell);
                    if let Some(cell_ref) = buf.cell_mut((offset_x + x, offset_y + y)) {
                        cell_ref.set_char(ch);
                        cell_ref.set_style(style);
                    }
                }
            }
        }
    }
}
