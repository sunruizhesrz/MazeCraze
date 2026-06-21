use crossterm::event::KeyCode;

use crate::core::Direction;

use super::app::{App, AppState};

/// Commands that the event handler can return.
pub enum Command {
    Quit,
    Continue,
}

/// Handle keyboard input based on current application state.
pub fn handle_key(app: &mut App, key: KeyCode) -> Command {
    match app.state {
        AppState::Menu => handle_menu_keys(app, key),
        AppState::Generating | AppState::Solving => handle_animation_keys(app, key),
        AppState::Playing => handle_playing_keys(app, key),
        AppState::Results => handle_results_keys(app, key),
        AppState::Help => handle_help_keys(app, key),
        AppState::Quit => Command::Quit,
    }
}

fn handle_menu_keys(app: &mut App, key: KeyCode) -> Command {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => Command::Quit,
        KeyCode::Char('h') => {
            app.state = AppState::Help;
            Command::Continue
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.prev_menu_item();
            Command::Continue
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.next_menu_item();
            Command::Continue
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.menu_action_right();
            Command::Continue
        }
        KeyCode::Left => {
            app.menu_action_left();
            Command::Continue
        }
        KeyCode::Char('g') => {
            app.generate_maze();
            Command::Continue
        }
        KeyCode::Char('s') => {
            app.solve_maze();
            Command::Continue
        }
        KeyCode::Char('p') => {
            app.start_challenge();
            Command::Continue
        }
        KeyCode::Enter => {
            app.menu_action_enter();
            Command::Continue
        }
        _ => Command::Continue,
    }
}

fn handle_animation_keys(app: &mut App, key: KeyCode) -> Command {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.state = AppState::Menu,
        KeyCode::Char(' ') => app.toggle_playback(),
        KeyCode::Right | KeyCode::Char('l') => app.step_forward(),
        KeyCode::Left | KeyCode::Char('h') => app.step_backward(),
        KeyCode::Char(']') | KeyCode::Char('+') => app.increase_speed(),
        KeyCode::Char('r') => {
            if app.state == AppState::Generating {
                app.generate_maze();
            } else {
                app.solve_maze();
            }
        }
        KeyCode::Char('n') => {
            if app.state == AppState::Generating {
                app.solve_maze();
            }
        }
        _ => {}
    }
    Command::Continue
}

fn handle_playing_keys(app: &mut App, key: KeyCode) -> Command {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.state = AppState::Menu,
        KeyCode::Char('r') => app.start_challenge(),
        KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => app.move_player(Direction::North),
        KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
            app.move_player(Direction::South)
        }
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => app.move_player(Direction::West),
        KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
            app.move_player(Direction::East)
        }
        _ => {}
    }
    Command::Continue
}

fn handle_results_keys(app: &mut App, key: KeyCode) -> Command {
    match key {
        KeyCode::Char('q') | KeyCode::Esc => app.state = AppState::Menu,
        KeyCode::Char('g') => app.generate_maze(),
        KeyCode::Char('s') => app.solve_maze(),
        KeyCode::Char('p') => app.start_challenge(),
        _ => {}
    }
    Command::Continue
}

fn handle_help_keys(app: &mut App, key: KeyCode) -> Command {
    match key {
        KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('h') => app.state = AppState::Menu,
        _ => {}
    }
    Command::Continue
}
