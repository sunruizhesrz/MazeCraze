use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Widget},
};

/// 主菜单使用的可选列表组件。
pub struct MenuList<'a> {
    items: Vec<&'a str>,
    selected: usize,
    title: &'a str,
}

impl<'a> MenuList<'a> {
    pub fn new(items: Vec<&'a str>, selected: usize, title: &'a str) -> Self {
        Self {
            items,
            selected,
            title,
        }
    }
}

impl<'a> Widget for MenuList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == self.selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(Line::from(*item)).style(style)
            })
            .collect();

        let list = List::new(items).block(Block::default().borders(Borders::ALL).title(self.title));
        list.render(area, buf);
    }
}
