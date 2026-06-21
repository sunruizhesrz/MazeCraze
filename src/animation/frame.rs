use crate::core::Grid;

/// A single frame in an animation sequence.
///
/// Each frame is a snapshot of the maze grid at a specific step,
/// along with a human-readable description of what happened.
#[derive(Clone, Debug)]
pub struct Frame {
    /// The grid state at this step.
    pub grid: Grid,
    /// Step number (0-indexed).
    pub step_number: usize,
    /// Human-readable description of the action.
    pub description: String,
}

impl Frame {
    /// Create a new frame.
    pub fn new(grid: Grid, step_number: usize, description: impl Into<String>) -> Self {
        Self {
            grid,
            step_number,
            description: description.into(),
        }
    }
}
