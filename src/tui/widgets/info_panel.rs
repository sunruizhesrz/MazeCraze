use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::animation::{AnimationPlayer, PlaybackState};

/// 显示动画元数据和控件的信息面板。
pub struct InfoPanel<'a> {
    player: &'a AnimationPlayer,
    message: &'a str,
}

impl<'a> InfoPanel<'a> {
    pub fn new(player: &'a AnimationPlayer, message: &'a str) -> Self {
        Self { player, message }
    }
}

impl<'a> Widget for InfoPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title("Info");
        let inner = block.inner(area);
        block.render(area, buf);

        let state_color = match self.player.state() {
            PlaybackState::Playing => Color::Green,
            PlaybackState::Paused => Color::Yellow,
            PlaybackState::Finished => Color::Cyan,
        };

        let text = vec![
            Line::from(vec![
                Span::raw("Frame: "),
                Span::styled(
                    format!(
                        "{} / {}",
                        self.player.current_frame(),
                        self.player.total_frames()
                    ),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::raw("State: "),
                Span::styled(
                    format!("{:?}", self.player.state()),
                    Style::default().fg(state_color),
                ),
            ]),
            Line::from(vec![
                Span::raw("Speed: "),
                Span::styled(
                    format!("{:?}", self.player.speed()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                self.message,
                Style::default().fg(Color::Yellow),
            )),
        ];

        Paragraph::new(text).render(inner, buf);
    }
}
