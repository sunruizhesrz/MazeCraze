use std::{fs, time::Duration};

use mazecraze::{
    core::{Cell, Direction, Grid, Point},
    tui::app::{App, AppState, BestRecord, LocalRecords},
};

#[test]
fn difficulty_selection_updates_maze_size() {
    let mut app = App::with_records(LocalRecords::default());

    assert_eq!((app.maze_width, app.maze_height), (21, 13));

    app.next_difficulty();
    assert_eq!(app.current_difficulty_name(), "Hard");
    assert_eq!((app.maze_width, app.maze_height), (31, 19));

    app.next_difficulty();
    assert_eq!(app.current_difficulty_name(), "Easy");
    assert_eq!((app.maze_width, app.maze_height), (15, 9));
}

#[test]
fn player_moves_only_through_passages() {
    let mut grid = Grid::new(5, 5).unwrap();
    grid.set(Point::new(1, 1), Cell::Current).unwrap();
    grid.set(Point::new(2, 1), Cell::Passage).unwrap();

    let mut app = App::with_records(LocalRecords::default());
    app.state = AppState::Playing;
    app.maze_width = 5;
    app.maze_height = 5;
    app.player_position = Point::new(1, 1);
    app.play_grid = Some(grid);

    app.move_player(Direction::East);
    assert_eq!(app.player_position, Point::new(2, 1));
    assert_eq!(app.challenge_steps, 1);

    app.move_player(Direction::South);
    assert_eq!(app.player_position, Point::new(2, 1));
    assert_eq!(app.challenge_steps, 1);
}

#[test]
fn local_records_keep_the_fastest_result() {
    let path =
        std::env::temp_dir().join(format!("mazecraze_records_test_{}.txt", std::process::id()));
    let _ = fs::remove_file(&path);

    let mut records = LocalRecords::default();
    assert!(records.set_if_better(
        "Easy|15x9|Recursive Backtracker".to_string(),
        BestRecord {
            elapsed_millis: Duration::from_secs(12).as_millis(),
            steps: 44,
        },
    ));
    assert!(!records.set_if_better(
        "Easy|15x9|Recursive Backtracker".to_string(),
        BestRecord {
            elapsed_millis: Duration::from_secs(15).as_millis(),
            steps: 30,
        },
    ));
    assert!(records.set_if_better(
        "Easy|15x9|Recursive Backtracker".to_string(),
        BestRecord {
            elapsed_millis: Duration::from_secs(12).as_millis(),
            steps: 40,
        },
    ));

    records.save_to_path(&path).unwrap();
    let loaded = LocalRecords::load_from_path(&path);
    assert_eq!(
        loaded.get("Easy|15x9|Recursive Backtracker"),
        Some(BestRecord {
            elapsed_millis: 12_000,
            steps: 40,
        })
    );

    let _ = fs::remove_file(path);
}
