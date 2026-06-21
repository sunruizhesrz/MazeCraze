//! Core data structures for the maze.
//!
//! This module defines the fundamental building blocks: grid cells, coordinates,
//! and directions. All other modules depend on these primitives.

pub mod direction;
pub mod grid;
pub mod point;

pub use direction::Direction;
pub use grid::{Cell, Grid};
pub use point::Point;
