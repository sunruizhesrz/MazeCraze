use crate::core::Grid;

use super::Frame;

/// 记录迷宫生成或求解过程的每一步。
///
/// 记录器会在关键时机对 [`Grid`] 进行快照，生成一系列 [`Frame`]，
/// 可作为动画回放。
#[derive(Clone, Debug)]
pub struct AnimationRecorder {
    frames: Vec<Frame>,
    initial_grid: Grid,
}

impl AnimationRecorder {
    /// 创建一个新的记录器，使用指定尺寸的空网格。
    pub fn new(width: usize, height: usize) -> Self {
        let grid = Grid::new(width, height).expect("valid dimensions");
        Self {
            frames: Vec::new(),
            initial_grid: grid.clone(),
        }
    }

    /// 从已有网格创建记录器（用于求解器）。
    pub fn from_grid(grid: &Grid) -> Self {
        Self {
            frames: Vec::new(),
            initial_grid: grid.clone(),
        }
    }

    /// 将给定网格的快照记录为新的一帧。
    pub fn record(&mut self, grid: &Grid, description: impl Into<String>) {
        self.frames
            .push(Frame::new(grid.clone(), self.frames.len(), description));
    }

    /// 以最终帧标记动画结束。
    pub fn finish(&mut self, grid: Grid) {
        self.frames
            .push(Frame::new(grid, self.frames.len(), "Finished"));
    }

    /// 访问所有已记录的帧。
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// 已记录的步骤总数。
    pub fn total_steps(&self) -> usize {
        self.frames.len()
    }

    /// 获取初始网格（在任何修改之前）。
    pub fn initial_grid(&self) -> &Grid {
        &self.initial_grid
    }

    /// 消耗记录器并返回其中的帧。
    pub fn into_frames(self) -> Vec<Frame> {
        self.frames
    }
}
