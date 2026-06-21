use crate::animation::AnimationPlayer;
use crate::core::{Cell, Direction, Grid, Point};
use crate::generator::{all_generators, MazeGenerator};
use crate::renderer::{all_renderers, Renderer};
use crate::solver::{all_solvers, MazeSolver};
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

/// Top-level application state machine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu,
    Generating,
    Solving,
    Playing,
    Results,
    Help,
    Quit,
}

/// Number of selectable menu items.
pub const MENU_ITEM_COUNT: usize = 6;

/// Preset challenge difficulties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub const ALL: [Self; 3] = [Self::Easy, Self::Normal, Self::Hard];

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }

    pub const fn size(&self) -> (usize, usize) {
        match self {
            Self::Easy => (15, 9),
            Self::Normal => (21, 13),
            Self::Hard => (31, 19),
        }
    }
}

/// A best local challenge result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BestRecord {
    pub elapsed_millis: u128,
    pub steps: usize,
}

impl BestRecord {
    fn is_better_than(&self, other: &Self) -> bool {
        self.elapsed_millis < other.elapsed_millis
            || (self.elapsed_millis == other.elapsed_millis && self.steps < other.steps)
    }
}

/// Dependency-free local best-record storage.
#[derive(Clone, Debug, Default)]
pub struct LocalRecords {
    entries: Vec<(String, BestRecord)>,
}

impl LocalRecords {
    const FILE_NAME: &'static str = "mazecraze_records.txt";

    pub fn load_default() -> Self {
        Self::load_from_path(Self::FILE_NAME)
    }

    pub fn load_from_path(path: impl AsRef<Path>) -> Self {
        let Ok(content) = fs::read_to_string(path) else {
            return Self::default();
        };

        let mut records = Self::default();
        for line in content.lines() {
            let mut parts = line.split('\t');
            let Some(key) = parts.next() else { continue };
            let Some(millis) = parts.next().and_then(|v| v.parse().ok()) else {
                continue;
            };
            let Some(steps) = parts.next().and_then(|v| v.parse().ok()) else {
                continue;
            };
            records.set_if_better(
                key.to_string(),
                BestRecord {
                    elapsed_millis: millis,
                    steps,
                },
            );
        }
        records
    }

    pub fn save_default(&self) -> std::io::Result<()> {
        self.save_to_path(Self::FILE_NAME)
    }

    pub fn save_to_path(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let content = self
            .entries
            .iter()
            .map(|(key, record)| format!("{key}\t{}\t{}", record.elapsed_millis, record.steps))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(path, content)
    }

    pub fn get(&self, key: &str) -> Option<BestRecord> {
        self.entries
            .iter()
            .find(|(entry_key, _)| entry_key == key)
            .map(|(_, record)| *record)
    }

    pub fn set_if_better(&mut self, key: String, record: BestRecord) -> bool {
        if let Some((_, existing)) = self
            .entries
            .iter_mut()
            .find(|(entry_key, _)| entry_key == &key)
        {
            if record.is_better_than(existing) {
                *existing = record;
                return true;
            }
            return false;
        }

        self.entries.push((key, record));
        true
    }
}

/// Main application state.
pub struct App {
    pub state: AppState,
    pub selected_menu_item: usize,
    pub selected_generator: usize,
    pub selected_solver: usize,
    pub selected_renderer: usize,
    pub selected_difficulty: usize,
    pub maze_width: usize,
    pub maze_height: usize,
    pub loop_rate: f64,
    pub player: Option<AnimationPlayer>,
    pub current_grid: Option<Grid>,
    pub play_grid: Option<Grid>,
    pub player_position: Point,
    pub challenge_start: Option<Instant>,
    pub challenge_elapsed: Duration,
    pub challenge_steps: usize,
    pub challenge_finished: bool,
    pub best_record: Option<BestRecord>,
    pub records: LocalRecords,
    pub message: String,
}

impl App {
    pub fn new() -> Self {
        Self::with_records(LocalRecords::load_default())
    }

    pub fn with_records(records: LocalRecords) -> Self {
        let difficulty = Difficulty::Normal;
        let (maze_width, maze_height) = difficulty.size();
        Self {
            state: AppState::Menu,
            selected_menu_item: 0,
            selected_generator: 0,
            selected_solver: 0,
            selected_renderer: 0,
            selected_difficulty: 1,
            maze_width,
            maze_height,
            loop_rate: 0.0,
            player: None,
            current_grid: None,
            play_grid: None,
            player_position: Point::new(1, 1),
            challenge_start: None,
            challenge_elapsed: Duration::ZERO,
            challenge_steps: 0,
            challenge_finished: false,
            best_record: None,
            records,
            message: String::from("Welcome to MazeCraze! Press 'h' for help."),
        }
    }

    pub fn next_menu_item(&mut self) {
        self.selected_menu_item = (self.selected_menu_item + 1) % MENU_ITEM_COUNT;
    }

    pub fn prev_menu_item(&mut self) {
        self.selected_menu_item = (self.selected_menu_item + MENU_ITEM_COUNT - 1) % MENU_ITEM_COUNT;
    }

    pub fn menu_action_right(&mut self) {
        match self.selected_menu_item {
            0 => self.next_difficulty(),
            1 => self.next_generator(),
            2 => self.next_solver(),
            _ => {}
        }
    }

    pub fn menu_action_left(&mut self) {
        match self.selected_menu_item {
            0 => self.prev_difficulty(),
            1 => self.prev_generator(),
            2 => self.prev_solver(),
            _ => {}
        }
    }

    pub fn menu_action_enter(&mut self) {
        match self.selected_menu_item {
            3 => self.start_challenge(),
            4 => self.generate_maze(),
            5 => self.solve_maze(),
            _ => {}
        }
    }

    pub fn current_difficulty(&self) -> Difficulty {
        Difficulty::ALL[self.selected_difficulty]
    }

    pub fn current_difficulty_name(&self) -> &'static str {
        self.current_difficulty().name()
    }

    pub fn next_difficulty(&mut self) {
        self.selected_difficulty = (self.selected_difficulty + 1) % Difficulty::ALL.len();
        self.apply_difficulty_size();
    }

    pub fn prev_difficulty(&mut self) {
        self.selected_difficulty =
            (self.selected_difficulty + Difficulty::ALL.len() - 1) % Difficulty::ALL.len();
        self.apply_difficulty_size();
    }

    fn apply_difficulty_size(&mut self) {
        let (width, height) = self.current_difficulty().size();
        self.maze_width = width;
        self.maze_height = height;
    }

    pub fn next_generator(&mut self) {
        let count = self.generators().len();
        if count > 0 {
            self.selected_generator = (self.selected_generator + 1) % count;
        }
    }

    pub fn prev_generator(&mut self) {
        let count = self.generators().len();
        if count > 0 {
            self.selected_generator = (self.selected_generator + count - 1) % count;
        }
    }

    pub fn next_solver(&mut self) {
        let count = self.solvers().len();
        if count > 0 {
            self.selected_solver = (self.selected_solver + 1) % count;
        }
    }

    pub fn prev_solver(&mut self) {
        let count = self.solvers().len();
        if count > 0 {
            self.selected_solver = (self.selected_solver + count - 1) % count;
        }
    }

    pub fn increase_size(&mut self) {
        if self.maze_width < 51 {
            self.maze_width += 2;
        }
        if self.maze_height < 51 {
            self.maze_height += 2;
        }
    }

    pub fn decrease_size(&mut self) {
        if self.maze_width > 5 {
            self.maze_width -= 2;
        }
        if self.maze_height > 5 {
            self.maze_height -= 2;
        }
    }

    pub fn generators(&self) -> Vec<Box<dyn MazeGenerator>> {
        all_generators()
    }

    pub fn solvers(&self) -> Vec<Box<dyn MazeSolver>> {
        all_solvers()
    }

    pub fn renderers(&self) -> Vec<Box<dyn Renderer>> {
        all_renderers()
    }

    pub fn current_generator_name(&self) -> String {
        let gens = self.generators();
        gens.get(self.selected_generator)
            .map(|g| g.name().to_string())
            .unwrap_or_default()
    }

    pub fn current_solver_name(&self) -> String {
        let sols = self.solvers();
        sols.get(self.selected_solver)
            .map(|s| s.name().to_string())
            .unwrap_or_default()
    }

    pub fn generate_maze(&mut self) {
        let gens = self.generators();
        if let Some(generator) = gens.get(self.selected_generator) {
            self.message = format!("Generating maze with {}...", generator.name());
            let recorder = generator.generate(self.maze_width, self.maze_height);
            self.current_grid = Some(recorder.frames().last().unwrap().grid.clone());
            self.player = Some(AnimationPlayer::new(recorder));
            self.state = AppState::Generating;
        }
    }

    pub fn solve_maze(&mut self) {
        if let Some(ref grid) = self.current_grid {
            let sols = self.solvers();
            if let Some(solver) = sols.get(self.selected_solver) {
                self.message = format!("Solving with {}...", solver.name());
                let start = Point::new(1, 1);
                let end = Point::new(grid.width() - 2, grid.height() - 2);
                let recorder = solver.solve(grid, start, end);
                self.player = Some(AnimationPlayer::new(recorder));
                self.state = AppState::Solving;
            }
        } else {
            self.message = String::from("Generate a maze first!");
        }
    }

    pub fn start_challenge(&mut self) {
        let gens = self.generators();
        if let Some(generator) = gens.get(self.selected_generator) {
            let recorder = generator.generate(self.maze_width, self.maze_height);
            let mut grid = recorder.frames().last().unwrap().grid.clone();
            grid.clear_markers();
            self.player_position = Point::new(1, 1);
            let _ = grid.set(self.player_position, Cell::Current);
            self.current_grid = Some(grid.clone());
            self.play_grid = Some(grid);
            self.challenge_start = Some(Instant::now());
            self.challenge_elapsed = Duration::ZERO;
            self.challenge_steps = 0;
            self.challenge_finished = false;
            self.best_record = self.records.get(&self.current_record_key());
            self.message = format!(
                "{} challenge started. Reach the exit at ({}, {}).",
                self.current_difficulty_name(),
                self.maze_width - 2,
                self.maze_height - 2
            );
            self.state = AppState::Playing;
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        if self.state != AppState::Playing || self.challenge_finished {
            return;
        }

        let Some(next) = self.player_position.neighbor(direction) else {
            return;
        };

        let Some(ref mut grid) = self.play_grid else {
            return;
        };

        if !grid.contains(next) || matches!(grid.get(next), Some(Cell::Wall) | None) {
            self.message = String::from("Blocked by a wall.");
            return;
        }

        let _ = grid.set(self.player_position, Cell::Visited);
        self.player_position = next;
        self.challenge_steps += 1;
        let _ = grid.set(self.player_position, Cell::Current);

        if self.player_position == self.challenge_end() {
            self.finish_challenge();
        } else {
            self.message = format!("Steps: {}", self.challenge_steps);
        }
    }

    fn finish_challenge(&mut self) {
        self.update_challenge_elapsed();
        self.challenge_finished = true;
        self.challenge_start = None;
        let record = BestRecord {
            elapsed_millis: self.challenge_elapsed.as_millis(),
            steps: self.challenge_steps,
        };
        let key = self.current_record_key();
        let improved = self.records.set_if_better(key, record);
        if improved {
            let _ = self.records.save_default();
        }
        self.best_record = self.records.get(&self.current_record_key());
        self.message = if improved {
            String::from("Finished! New best record saved.")
        } else {
            String::from("Finished!")
        };
        self.state = AppState::Results;
    }

    pub fn challenge_end(&self) -> Point {
        Point::new(self.maze_width - 2, self.maze_height - 2)
    }

    pub fn current_record_key(&self) -> String {
        format!(
            "{}|{}x{}|{}",
            self.current_difficulty_name(),
            self.maze_width,
            self.maze_height,
            self.current_generator_name()
        )
    }

    pub fn formatted_elapsed(&self) -> String {
        let total_millis = self.challenge_elapsed.as_millis();
        let seconds = total_millis / 1_000;
        let millis = total_millis % 1_000;
        format!("{seconds}.{millis:03}s")
    }

    pub fn format_record(record: BestRecord) -> String {
        let seconds = record.elapsed_millis / 1_000;
        let millis = record.elapsed_millis % 1_000;
        format!("{seconds}.{millis:03}s, {} steps", record.steps)
    }

    fn update_challenge_elapsed(&mut self) {
        if let Some(start) = self.challenge_start {
            self.challenge_elapsed = start.elapsed();
        }
    }

    pub fn toggle_playback(&mut self) {
        if let Some(ref mut player) = self.player {
            player.toggle_playback();
        }
    }

    pub fn step_forward(&mut self) {
        if let Some(ref mut player) = self.player {
            player.step_forward();
        }
    }

    pub fn step_backward(&mut self) {
        if let Some(ref mut player) = self.player {
            player.step_backward();
        }
    }

    pub fn increase_speed(&mut self) {
        if let Some(ref mut player) = self.player {
            player.set_speed(player.speed().next());
        }
    }

    pub fn on_tick(&mut self) {
        if let Some(ref mut player) = self.player {
            if self.state == AppState::Generating || self.state == AppState::Solving {
                player.tick();
            }
        }
        if self.state == AppState::Playing {
            self.update_challenge_elapsed();
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
