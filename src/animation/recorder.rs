use crate::core::Grid;

use super::Frame;

/// Records every step of a maze generation or solving process.
///
/// The recorder takes snapshots of a [`Grid`] at key moments, producing
/// a sequence of [`Frame`]s that can be played back as an animation.
#[derive(Clone, Debug)]
pub struct AnimationRecorder {
    frames: Vec<Frame>,
    initial_grid: Grid,
}

impl AnimationRecorder {
    /// Create a new recorder with an empty grid of the given size.
    pub fn new(width: usize, height: usize) -> Self {
        let grid = Grid::new(width, height).expect("valid dimensions");
        Self {
            frames: Vec::new(),
            initial_grid: grid.clone(),
        }
    }

    /// Create a recorder from an existing grid (for solvers).
    pub fn from_grid(grid: &Grid) -> Self {
        Self {
            frames: Vec::new(),
            initial_grid: grid.clone(),
        }
    }

    /// Record a snapshot of the given grid as a new frame.
    pub fn record(&mut self, grid: &Grid, description: impl Into<String>) {
        self.frames
            .push(Frame::new(grid.clone(), self.frames.len(), description));
    }

    /// Mark the end of the animation with a final frame.
    pub fn finish(&mut self, grid: Grid) {
        self.frames
            .push(Frame::new(grid, self.frames.len(), "Finished"));
    }

    /// Access all recorded frames.
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// Total number of steps recorded.
    pub fn total_steps(&self) -> usize {
        self.frames.len()
    }

    /// Get the initial grid (before any modifications).
    pub fn initial_grid(&self) -> &Grid {
        &self.initial_grid
    }

    /// Consume the recorder and return the frames.
    pub fn into_frames(self) -> Vec<Frame> {
        self.frames
    }
}
