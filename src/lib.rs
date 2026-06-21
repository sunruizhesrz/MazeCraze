//! MazeCraze — A TUI interactive maze generator and solver.
//!
//! This library provides:
//! - Multiple maze generation algorithms (Recursive Backtracker, Prim, Kruskal)
//! - Multiple maze solving algorithms (BFS, DFS, A*, Wall Follower)
//! - Real-time step-by-step animation recording and playback
//! - Unicode and ASCII rendering engines
//! - Terminal UI for interactive exploration

pub mod animation;
pub mod cli;
pub mod core;
pub mod export;
pub mod generator;
pub mod renderer;
pub mod solver;
pub mod tui;

/// Re-export commonly used types for convenience.
pub use core::{grid::Grid, point::Point};
