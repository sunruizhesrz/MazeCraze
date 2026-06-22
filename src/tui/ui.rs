use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use super::app::{App, AppState};
use super::widgets::maze_canvas::MazeCanvas;

/// 根据当前应用状态绘制 UI。
pub fn draw(f: &mut Frame, app: &App) {
    match app.state {
        AppState::Menu => draw_menu(f, app),
        AppState::Generating | AppState::Solving => draw_animation(f, app),
        AppState::Playing => draw_playing(f, app),
        AppState::Results => draw_results(f, app),
        AppState::Help => draw_help(f),
        AppState::Quit => {}
    }
}

fn draw_menu(f: &mut Frame, app: &App) {
    let area = f.area();

    // 主纵向布局：标题 / 菜单 / 页脚
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(4), // 标题块
            Constraint::Min(8),    // 菜单
            Constraint::Length(3), // 页脚
        ])
        .split(area);

    // 标题
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan))
        .title(" MazeCraze ");
    let title_text = Text::from(vec![
        Line::from(Span::styled(
            "Interactive Maze Generator & Solver",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Rust TUI Edition",
            Style::default().fg(Color::Gray),
        )),
    ]);
    let title = Paragraph::new(title_text).alignment(Alignment::Center);
    f.render_widget(title_block.clone(), chunks[0]);
    let inner = title_block.inner(chunks[0]);
    f.render_widget(title, inner);

    // 菜单项
    let menu_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
        .title(" Menu ");

    let gen_name = app.current_generator_name();
    let sol_name = app.current_solver_name();
    let difficulty_str = format!(
        "{} ({} x {})",
        app.current_difficulty_name(),
        app.maze_width,
        app.maze_height
    );

    let items: Vec<(&str, &str)> = vec![
        ("Difficulty", difficulty_str.as_str()),
        ("Generator", gen_name.as_str()),
        ("Solver", sol_name.as_str()),
        ("Action", "Start challenge"),
        ("Action", "Watch generation"),
        ("Action", "Watch solver"),
    ];

    let mut menu_lines: Vec<Line> = Vec::new();
    for (i, (label, value)) in items.iter().enumerate() {
        let is_selected = i == app.selected_menu_item;
        let prefix = if is_selected { "▶ " } else { "  " };
        let style = if is_selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        menu_lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{:<12} ", label), style),
            Span::styled(value.to_string(), style),
        ]));
    }

    let menu = Paragraph::new(Text::from(menu_lines));
    f.render_widget(menu_block.clone(), chunks[1]);
    let menu_inner = menu_block.inner(chunks[1]);
    f.render_widget(menu, menu_inner);

    // 页脚 / 状态
    let footer = Paragraph::new(app.message.as_str())
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );
    f.render_widget(footer, chunks[2]);
}

fn draw_animation(f: &mut Frame, app: &App) {
    let area = f.area();

    // 纵向分割：上方迷宫（大区域），下方信息栏
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // 迷宫区域
            Constraint::Length(7), // 信息 / 控制区
        ])
        .split(area);

    let maze_area = chunks[0];
    let info_area = chunks[1];

    // 迷宫显示
    let maze_title = if app.state == AppState::Generating {
        " Generating "
    } else {
        " Solving "
    };
    let maze_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title(maze_title);
    f.render_widget(maze_block.clone(), maze_area);

    let maze_inner = maze_block.inner(maze_area);

    if let Some(ref player) = app.player {
        if let Some(frame) = player.frames().get(player.current_frame()) {
            let canvas = MazeCanvas::new(&frame.grid);
            f.render_widget(canvas, maze_inner);
        }
    } else {
        let empty = Paragraph::new("No animation loaded")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, maze_inner);
    }

    // 信息面板：双列布局
    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info_area);

    // 左侧：播放统计
    let stats_text = if let Some(ref player) = app.player {
        let pct = if player.total_frames() > 0 {
            (player.current_frame() * 100) / player.total_frames()
        } else {
            0
        };
        Text::from(vec![
            Line::from(vec![
                Span::styled("Frame: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{} / {}", player.current_frame(), player.total_frames()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("Progress: ", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}%", pct), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Speed: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:?}", player.speed()),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("State: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:?}", player.state()),
                    Style::default().fg(Color::White),
                ),
            ]),
        ])
    } else {
        Text::from("No animation loaded")
    };

    let stats = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(" Playback "),
    );
    f.render_widget(stats, info_chunks[0]);

    // 右侧：控制说明
    let controls_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Space ", Style::default().fg(Color::Yellow)),
            Span::styled("Play/Pause", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("← / →  ", Style::default().fg(Color::Yellow)),
            Span::styled("Step back/forward", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("+      ", Style::default().fg(Color::Yellow)),
            Span::styled("Speed up", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("r      ", Style::default().fg(Color::Yellow)),
            Span::styled("Restart", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("q      ", Style::default().fg(Color::Yellow)),
            Span::styled("Menu", Style::default().fg(Color::White)),
        ]),
    ]);

    let controls = Paragraph::new(controls_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .title(" Controls "),
    );
    f.render_widget(controls, info_chunks[1]);
}

fn draw_playing(f: &mut Frame, app: &App) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(7)])
        .split(area);

    let maze_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
        .title(" Challenge ");
    f.render_widget(maze_block.clone(), chunks[0]);
    let maze_inner = maze_block.inner(chunks[0]);

    if let Some(ref grid) = app.play_grid {
        let canvas = MazeCanvas::new(grid);
        f.render_widget(canvas, maze_inner);
    } else {
        let empty = Paragraph::new("No challenge loaded")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(empty, maze_inner);
    }

    let info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let best = app
        .best_record
        .map(App::format_record)
        .unwrap_or_else(|| String::from("No record yet"));
    let stats_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.current_difficulty_name(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Time: ", Style::default().fg(Color::Gray)),
            Span::styled(app.formatted_elapsed(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Steps: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.challenge_steps.to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Best: ", Style::default().fg(Color::Gray)),
            Span::styled(best, Style::default().fg(Color::Yellow)),
        ]),
    ]);

    let stats = Paragraph::new(stats_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(" Challenge Stats "),
    );
    f.render_widget(stats, info_chunks[0]);

    let controls_text = Text::from(vec![
        Line::from(vec![
            Span::styled("Arrows/WASD ", Style::default().fg(Color::Yellow)),
            Span::styled("Move", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("r           ", Style::default().fg(Color::Yellow)),
            Span::styled("Restart challenge", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("q / Esc     ", Style::default().fg(Color::Yellow)),
            Span::styled("Menu", Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(app.message.as_str()),
    ]);

    let controls = Paragraph::new(controls_text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .title(" Controls "),
    );
    f.render_widget(controls, info_chunks[1]);
}

fn draw_results(f: &mut Frame, app: &App) {
    let area = f.area();

    let block = Block::default().borders(Borders::ALL).title(" Results ");
    f.render_widget(block.clone(), area);
    let inner = block.inner(area);

    let mut lines = vec![
        Line::from(Span::styled(
            "Maze Results",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    if app.challenge_finished {
        lines.push(Line::from(vec![
            Span::styled("Difficulty: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.current_difficulty_name(),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Time: ", Style::default().fg(Color::Gray)),
            Span::styled(app.formatted_elapsed(), Style::default().fg(Color::White)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Steps: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app.challenge_steps.to_string(),
                Style::default().fg(Color::White),
            ),
        ]));
        if let Some(record) = app.best_record {
            lines.push(Line::from(vec![
                Span::styled("Best: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    App::format_record(record),
                    Style::default().fg(Color::Yellow),
                ),
            ]));
        }
    } else if let Some(ref player) = app.player {
        let frame = player.frames().last().unwrap();
        lines.push(Line::from(vec![
            Span::styled("Total steps: ", Style::default().fg(Color::Gray)),
            Span::styled(
                player.total_frames().to_string(),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Message: ", Style::default().fg(Color::Gray)),
            Span::styled(frame.description.clone(), Style::default().fg(Color::White)),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press [p] to play again or [q] to return to menu",
        Style::default().fg(Color::Yellow),
    )));

    let text = Paragraph::new(Text::from(lines)).alignment(Alignment::Center);
    f.render_widget(text, inner);
}

fn draw_help(f: &mut Frame) {
    let area = f.area();

    let help_text = Text::from(vec![
        Line::from(Span::styled(
            "MazeCraze Keyboard Shortcuts",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Menu:", Style::default().fg(Color::Yellow))),
        Line::from("  ↑ / k      Move up"),
        Line::from("  ↓ / j      Move down"),
        Line::from("  ← / h      Adjust value down"),
        Line::from("  → / l      Adjust value up"),
        Line::from("  Enter      Trigger action"),
        Line::from("  p          Start challenge"),
        Line::from("  h          Toggle this help"),
        Line::from("  q / Esc    Quit"),
        Line::from(""),
        Line::from(Span::styled(
            "Challenge:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from("  Arrows/WASD Move player"),
        Line::from("  r           Restart challenge"),
        Line::from("  q / Esc     Return to menu"),
        Line::from(""),
        Line::from(Span::styled(
            "Animation:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from("  Space      Play / Pause"),
        Line::from("  → / l      Step forward"),
        Line::from("  ← / h      Step backward"),
        Line::from("  + / ]      Increase speed"),
        Line::from("  r          Restart"),
        Line::from("  q / Esc    Return to menu"),
    ]);

    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title(" Help "))
        .alignment(Alignment::Left);

    let popup_area = centered_rect(60, 70, area);
    f.render_widget(Clear, popup_area);
    f.render_widget(help, popup_area);
}

/// 根据给定百分比尺寸创建居中的矩形区域。
fn centered_rect(
    percent_x: u16,
    percent_y: u16,
    r: ratatui::layout::Rect,
) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
